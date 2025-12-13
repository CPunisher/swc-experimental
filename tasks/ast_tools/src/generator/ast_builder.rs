use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};

use crate::{
    AST_CRATE_PATH,
    output::{RawOutput, RustOutput, output_path},
    schema::{AstEnum, AstStruct, AstType, Schema},
    util::{InlineStorageMode, calculate_inline_layout, map_field_type_to_extra_field, safe_ident},
};

pub fn ast_builder(schema: &Schema) -> RawOutput {
    let mut build_functions = TokenStream::new();
    for ty in schema.types.iter() {
        match ty {
            AstType::Struct(ast_struct) => {
                build_functions.extend(generate_build_function_for_struct(ast_struct, schema))
            }
            AstType::Enum(ast_enum) => {
                let mut context = RecursiveEnumContext {
                    ret_ty: format_ident!("{}", ast_enum.name).to_token_stream(),
                    ..Default::default()
                };
                build_functions.extend(generate_build_function_for_enum(
                    ast_enum,
                    schema,
                    &mut context,
                ));
            }
            _ => continue,
        };
    }

    let output = quote! {
            #![allow(unused, clippy::useless_conversion)]
            use swc_core::common::Span;

            use crate::{Ast, AstNode, ExtraData, NodeData, NodeKind, ast::*, node_id::*};

            impl Ast {
                #build_functions
            }
    };

    RustOutput {
        path: output_path(AST_CRATE_PATH, "ast_builder"),
        tokens: output,
    }
    .into()
}

fn generate_build_function_for_struct(ast: &AstStruct, schema: &Schema) -> TokenStream {
    let fn_name = safe_ident(&ast.name.to_case(Case::Snake));
    let ret_ty = format_ident!("{}", ast.name);
    let fn_params = generate_fn_params_decl(ast, schema);

    // Check if this struct can use inline storage
    if let Some(layout) = calculate_inline_layout(ast, schema) {
        generate_build_function_inline(ast, schema, fn_name, ret_ty, fn_params, &layout)
    } else {
        generate_build_function_extra_data(ast, schema, fn_name, ret_ty, fn_params)
    }
}

/// Generate build function using inline storage (for small nodes)
/// Layout (7 bytes total):
/// - Bytes 0-3: NodeData.inline_data (u32)
/// - Bytes 4-6: AstNode.inline_data ([u8; 3])
fn generate_build_function_inline(
    ast: &AstStruct,
    schema: &Schema,
    fn_name: syn::Ident,
    ret_ty: syn::Ident,
    fn_params: TokenStream,
    layout: &crate::util::InlineLayout,
) -> TokenStream {
    // Optimization: zero-field nodes use empty: () for shorter generated code
    if layout.fields.is_empty() {
        return quote! {
            #[inline]
            pub fn #fn_name(&mut self, span: Span) -> #ret_ty {
                #ret_ty(self.add_node(AstNode {
                    span,
                    kind: NodeKind::#ret_ty,
                    inline_data: [0, 0, 0],
                    data: NodeData { empty: () },
                }))
            }
        };
    }

    // Optimization: if FourBytes mode with a single field at offset 0 (1-4 bytes),
    // we can directly assign the u32 value without byte conversion
    let is_single_u32_field = layout.mode == InlineStorageMode::FourBytes
        && layout.fields.len() == 1
        && layout.fields[0].1 == 0   // byte_offset == 0
        && layout.fields[0].2 <= 4;  // byte_size <= 4

    if is_single_u32_field {
        let (field_idx, _, _) = layout.fields[0];
        let field = &ast.fields[field_idx];
        let field_name = format_ident!("{}", field.name);
        let field_ty = &schema.types[field.type_id];

        // Generate direct u32 value
        let u32_value = generate_field_to_u32(field_ty, &field_name, schema);

        return quote! {
            #[inline]
            pub fn #fn_name(&mut self, span: Span, #fn_params) -> #ret_ty {
                #ret_ty(
                    self.add_node(AstNode {
                        span,
                        kind: NodeKind::#ret_ty,
                        inline_data: [0, 0, 0],
                        data: NodeData {
                            inline_data: #u32_value,
                        },
                    })
                )
            }
        };
    }

    // General case: pack fields into bytes
    let mut pack_code = TokenStream::new();

    for &(field_idx, byte_offset, byte_size) in &layout.fields {
        let field = &ast.fields[field_idx];
        let field_name = format_ident!("{}", field.name);
        let field_ty = &schema.types[field.type_id];

        // Generate bytes for this field
        let bytes_expr = generate_field_to_bytes(field_ty, &field_name, byte_size);

        // Store bytes in a temporary variable to avoid repeated evaluation
        let tmp_var = format_ident!("{}_bytes", field.name);
        pack_code.extend(quote! {
            let #tmp_var = #bytes_expr;
        });

        // Write bytes to inline_bytes array at the correct offset
        for i in 0..byte_size {
            let dst_idx = byte_offset + i;
            pack_code.extend(quote! {
                inline_bytes[#dst_idx] = #tmp_var[#i];
            });
        }
    }

    let tokens = match layout.mode {
        InlineStorageMode::FourBytes => {
            // Multiple small fields fit in the u32
            quote! {
                #[inline]
                pub fn #fn_name(&mut self, span: Span, #fn_params) -> #ret_ty {
                    let mut inline_bytes = [0u8; 4];
                    #pack_code

                    #ret_ty(
                        self.add_node(AstNode {
                            span,
                            kind: NodeKind::#ret_ty,
                            inline_data: [0, 0, 0],
                            data: NodeData {
                                inline_data: u32::from_le_bytes(inline_bytes),
                            },
                        })
                    )
                }
            }
        }
        InlineStorageMode::Full => {
            // Use all 7 bytes
            quote! {
                #[inline]
                pub fn #fn_name(&mut self, span: Span, #fn_params) -> #ret_ty {
                    let mut inline_bytes = [0u8; 7];
                    #pack_code

                    #ret_ty(
                        self.add_node(AstNode {
                            span,
                            kind: NodeKind::#ret_ty,
                            inline_data: [inline_bytes[4], inline_bytes[5], inline_bytes[6]],
                            data: NodeData {
                                inline_data: u32::from_le_bytes([inline_bytes[0], inline_bytes[1], inline_bytes[2], inline_bytes[3]]),
                            },
                        })
                    )
                }
            }
        }
    };

    tokens
}

/// Generate expression to convert a field value directly to u32 (for single field optimization, 1-4 bytes)
fn generate_field_to_u32(field_ty: &AstType, field_name: &syn::Ident, schema: &Schema) -> TokenStream {
    match field_ty {
        AstType::Struct(_) | AstType::Enum(_) => {
            quote!(#field_name.node_id().index() as u32)
        }
        AstType::Option(_) => {
            quote!(crate::OptionalNodeId::from(#field_name.map(|n| n.node_id())).into_raw())
        }
        AstType::Primitive(prim) => match prim.name {
            // 4-byte types
            "u32" => quote!(#field_name),
            "i32" => quote!(#field_name as u32),
            "BigIntId" => quote!(#field_name.index() as u32),
            // 2-byte types
            "u16" | "i16" => quote!(#field_name as u32),
            // 1-byte types
            "bool" => quote!(#field_name as u32),
            "u8" | "i8" => quote!(#field_name as u32),
            // Enums with #[repr(uN)] - just cast to u32
            name => {
                if let Some(&size) = schema.repr_sizes.get(name) {
                    assert!(size <= 4, "Cannot cast #[repr(u{})] enum to u32", size * 8);
                    quote!(#field_name as u32)
                } else {
                    unreachable!("Unexpected primitive in u32 conversion: {}", prim.name)
                }
            }
        },
        _ => unreachable!(),
    }
}

/// Generate expression to convert a field value to bytes
fn generate_field_to_bytes(
    field_ty: &AstType,
    field_name: &syn::Ident,
    _byte_size: usize,
) -> TokenStream {
    match field_ty {
        AstType::Struct(_) | AstType::Enum(_) => {
            quote!((#field_name.node_id().index() as u32).to_le_bytes())
        }
        AstType::Option(_) => {
            quote!(crate::OptionalNodeId::from(#field_name.map(|n| n.node_id())).into_raw().to_le_bytes())
        }
        AstType::Primitive(prim) => match prim.name {
            "bool" => quote!([#field_name as u8]),
            "u8" => quote!([#field_name]),
            "i8" => quote!([#field_name as u8]),
            "u16" | "i16" => quote!(#field_name.to_le_bytes()),
            "u32" | "i32" => quote!(#field_name.to_le_bytes()),
            "BigIntId" => quote!((#field_name.index() as u32).to_le_bytes()),
            // Small enums (1 byte)
            _ => quote!([(#field_name.to_extra_data() & 0xFF) as u8]),
        },
        _ => unreachable!("Unexpected field type for inline storage"),
    }
}

/// Generate build function using extra_data storage (for large nodes)
fn generate_build_function_extra_data(
    ast: &AstStruct,
    schema: &Schema,
    fn_name: syn::Ident,
    ret_ty: syn::Ident,
    fn_params: TokenStream,
) -> TokenStream {
    let mut add_extra_data = TokenStream::new();
    for (index, field) in ast.fields.iter().enumerate() {
        let extra_data_id = format_ident!("_f{}", index);
        let field_name = format_ident!("{}", field.name);

        let field_ty = &schema.types[field.type_id];
        let extra_data_field = map_field_type_to_extra_field(field_ty, schema);
        let field_value = match field_ty {
            AstType::Option(ast_option) => {
                let inner_ty = &schema.types[ast_option.inner_type_id];
                match inner_ty {
                    AstType::Vec(_) => quote!(#field_name.map(|n| n.inner).into()),
                    _ => quote!(#field_name.map(|n| n.node_id()).into()),
                }
            }
            AstType::Struct(_) | AstType::Enum(_) => {
                quote!( #field_name.node_id() )
            }
            _ if extra_data_field == "other" => quote!( #field_name.to_extra_data() ),
            _ => quote!( #field_name.into() ),
        };
        let extra_data_field = format_ident!("{}", extra_data_field);

        add_extra_data.extend(quote! {
            let #extra_data_id = self.add_extra(ExtraData { #extra_data_field: #field_value });
        });
    }

    let node_data = match ast.fields.len() {
        0 => quote!(empty: ()),
        _ => quote!(extra_data_start: _f0),
    };

    let tokens = quote! {
        #[inline]
        pub fn #fn_name(&mut self, span: Span, #fn_params) -> #ret_ty {
            #add_extra_data

            #ret_ty(
                self.add_node(AstNode {
                    span,
                    kind: NodeKind::#ret_ty,
                    inline_data: [0, 0, 0],
                    data: NodeData {
                        #node_data
                    },
                })
            )
        }
    };

    tokens
}

// TODO: Use [crate::util::flat_enum_type]
#[derive(Default)]
struct RecursiveEnumContext {
    ret_ty: TokenStream,
    name: Vec<String>,
    constructor: Vec<TokenStream>,
}

fn generate_build_function_for_enum(
    ast: &AstEnum,
    schema: &Schema,
    recursive_context: &mut RecursiveEnumContext,
) -> TokenStream {
    let mut build_variants = TokenStream::new();

    let enum_name = format_ident!("{}", ast.name);
    recursive_context.name.push(ast.name.clone());

    for variant in ast.variants.iter() {
        let Some(payload_ty_id) = variant.type_id else {
            continue;
        };

        let variant_name = format_ident!("{}", variant.name);
        recursive_context
            .constructor
            .push(quote!( #enum_name::#variant_name ));

        match &schema.types[payload_ty_id] {
            AstType::Struct(ast_struct) => {
                let ret_ty = &recursive_context.ret_ty;
                let fn_name = {
                    let mut fn_name = String::new();
                    for name in recursive_context.name.iter() {
                        fn_name.push_str(name);
                    }
                    fn_name.push_str(&ast_struct.name);
                    safe_ident(&fn_name.to_case(Case::Snake))
                };

                let args = generate_fn_args(ast_struct);
                let constructor = safe_ident(&ast_struct.name.to_case(Case::Snake));
                let body = recursive_context.constructor.iter().rev().fold(
                    quote!( self.#constructor(span, #args).into() ),
                    |acc, constructor| quote!(#constructor(#acc)),
                );

                let fn_params = generate_fn_params_decl(ast_struct, schema);
                build_variants.extend(quote! {
                    #[inline]
                    pub fn #fn_name(&mut self, span: Span, #fn_params) -> #ret_ty {
                        #body
                    }
                });
            }
            AstType::Enum(inner_enum) => {
                build_variants.extend(generate_build_function_for_enum(
                    inner_enum,
                    schema,
                    recursive_context,
                ));
            }
            _ => unreachable!(),
        }

        recursive_context.constructor.pop();
    }
    recursive_context.name.pop();

    build_variants
}

fn generate_fn_params_decl(ast: &AstStruct, schema: &Schema) -> TokenStream {
    let mut fields = Vec::default();
    for field in ast.fields.iter() {
        let field_name = format_ident!("{}", field.name);
        let field_ty = schema.types[field.type_id].repr_ident(schema);
        fields.push(quote!(#field_name: #field_ty));
    }

    quote!( #(#fields),* )
}

fn generate_fn_args(ast: &AstStruct) -> TokenStream {
    let mut fields = Vec::default();
    for field in ast.fields.iter() {
        let field_name = format_ident!("{}", field.name);
        fields.push(quote!(#field_name));
    }

    quote!( #(#fields),* )
}
