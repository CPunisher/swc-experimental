use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    AST_CRATE_PATH,
    output::{RawOutput, RustOutput, output_path},
    schema::{AstEnum, AstStruct, AstType, Schema},
    util::{
        INLINE_DATA_U32_SIZE, InlineLayout, InlineStorageMode, calculate_inline_layout,
        map_field_type_to_extra_field, safe_ident,
    },
};

pub fn ast_property(schema: &Schema) -> RawOutput {
    let mut impls = TokenStream::new();
    for ty in schema.types.iter() {
        match ty {
            AstType::Struct(ast_struct) => {
                impls.extend(generate_property_for_struct(ast_struct, schema))
            }
            AstType::Enum(ast_enum) => impls.extend(generate_property_for_enum(ast_enum, schema)),
            _ => continue,
        };
    }

    let output = quote! {
            #![allow(unused, clippy::useless_conversion)]
            use crate::{node_id::*, ast::*};

            #impls
    };

    RustOutput {
        path: output_path(AST_CRATE_PATH, "ast_property"),
        tokens: output,
    }
    .into()
}

fn generate_property_for_struct(ast: &AstStruct, schema: &Schema) -> TokenStream {
    let name = format_ident!("{}", ast.name);

    let mut field_getters = TokenStream::new();
    let mut field_setters = TokenStream::new();

    field_getters.extend(quote! {
        #[inline]
        pub fn span(&self, ast: &crate::Ast) -> crate::Span {
            unsafe { ast.nodes.get_unchecked(self.0).span }
        }
        #[inline]
        pub fn span_lo(&self, ast: &crate::Ast) -> crate::BytePos {
            self.span(ast).lo
        }
        #[inline]
        pub fn span_hi(&self, ast: &crate::Ast) -> crate::BytePos {
            self.span(ast).hi
        }
    });

    field_setters.extend(quote! {
        #[inline]
        pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
            unsafe { ast.nodes.get_unchecked_mut(self.0).span = span; }
        }
    });

    // Check if this struct uses inline storage
    if let Some(layout) = calculate_inline_layout(ast, schema) {
        generate_inline_property_accessors(ast, schema, &mut field_getters, &mut field_setters, &layout);
    } else {
        generate_extra_data_property_accessors(ast, schema, &mut field_getters, &mut field_setters);
    }

    quote! {
        impl #name {
            #field_getters
            #field_setters
        }
    }
}

/// Generate getters/setters for inline storage
/// Layout (7 bytes total):
/// - Bytes 0-3: NodeData.inline_data (u32)
/// - Bytes 4-6: AstNode.inline_data ([u8; 3])
fn generate_inline_property_accessors(
    ast: &AstStruct,
    schema: &Schema,
    field_getters: &mut TokenStream,
    field_setters: &mut TokenStream,
    layout: &InlineLayout,
) {
    for &(field_idx, byte_offset, byte_size) in &layout.fields {
        let field = &ast.fields[field_idx];
        let field_name = format_ident!("{}", field.name);
        let field_ty = &schema.types[field.type_id];
        let getter_name = safe_ident(&field.name.to_case(Case::Snake));
        let ret_ty = field_ty.repr_ident(schema);

        // Optimization: single field at offset 0 (1-4 bytes) can directly use u32
        let is_direct_u32 = byte_offset == 0 && byte_size <= 4
            && layout.mode == InlineStorageMode::FourBytes;

        if is_direct_u32 {
            // Generate optimized getter (direct u32 read)
            let cast_expr = generate_u32_to_type(field_ty, schema);
            field_getters.extend(quote! {
                #[inline]
                pub fn #getter_name(&self, ast: &crate::Ast) -> #ret_ty {
                    let node = unsafe { ast.nodes.get_unchecked(self.0) };
                    let raw = unsafe { node.data.inline_data };
                    #cast_expr
                }
            });

            // Generate optimized setter (direct u32 write)
            let to_u32_expr = generate_type_to_u32(field_ty, &field_name, schema);
            let setter_name = format_ident!("set_{}", field_name);
            field_setters.extend(quote! {
                #[inline]
                pub fn #setter_name(&self, ast: &mut crate::Ast, #field_name: #ret_ty) {
                    let node = unsafe { ast.nodes.get_unchecked_mut(self.0) };
                    node.data.inline_data = #to_u32_expr;
                }
            });
        } else {
            // General case: use byte packing/unpacking
            let read_expr = generate_inline_read(field_ty, schema, byte_offset, byte_size, &layout.mode);
            field_getters.extend(quote! {
                #[inline]
                pub fn #getter_name(&self, ast: &crate::Ast) -> #ret_ty {
                    let node = unsafe { ast.nodes.get_unchecked(self.0) };
                    #read_expr
                }
            });

            let setter_name = format_ident!("set_{}", field_name);
            let write_expr = generate_inline_write(field_ty, &field_name, byte_offset, byte_size, &layout.mode);
            field_setters.extend(quote! {
                #[inline]
                pub fn #setter_name(&self, ast: &mut crate::Ast, #field_name: #ret_ty) {
                    let node = unsafe { ast.nodes.get_unchecked_mut(self.0) };
                    #write_expr
                }
            });
        }
    }
}

/// Generate expression to convert u32 directly to target type (optimized path)
fn generate_u32_to_type(field_ty: &AstType, schema: &Schema) -> TokenStream {
    match field_ty {
        AstType::Struct(_) | AstType::Enum(_) => {
            let field_inner_ty = field_ty.repr_ident(schema);
            quote! {
                unsafe { #field_inner_ty::from_node_id_unchecked(crate::NodeId::from_usize(raw as usize), ast) }
            }
        }
        AstType::Option(ast_option) => {
            let inner_ty = &schema.types[ast_option.inner_type_id];
            let field_inner_ident = inner_ty.repr_ident(schema);
            quote! {
                let opt = crate::OptionalNodeId::from_raw(raw);
                opt.map(|id| unsafe { #field_inner_ident::from_node_id_unchecked(id, ast) })
            }
        }
        AstType::Primitive(prim) => {
            let ty_ident = format_ident!("{}", prim.name);
            match prim.name {
                // 4-byte types
                "u32" => quote! { raw },
                "i32" => quote! { raw as i32 },
                "BigIntId" => quote! { crate::BigIntId::from_usize(raw as usize) },
                // 2-byte types
                "u16" => quote! { raw as u16 },
                "i16" => quote! { raw as i16 },
                // 1-byte types
                "bool" => quote! { raw != 0 },
                "u8" => quote! { raw as u8 },
                "i8" => quote! { raw as i8 },
                // Enums with #[repr(uN)] - transmute from the appropriate size
                name => {
                    if let Some(&size) = schema.repr_sizes.get(name) {
                        match size {
                            1 => quote! { unsafe { std::mem::transmute::<u8, #ty_ident>(raw as u8) } },
                            2 => quote! { unsafe { std::mem::transmute::<u16, #ty_ident>(raw as u16) } },
                            4 => quote! { unsafe { std::mem::transmute::<u32, #ty_ident>(raw) } },
                            _ => unreachable!("Unsupported repr size: {}", size),
                        }
                    } else {
                        unreachable!("Unexpected primitive in u32 read: {}", prim.name)
                    }
                }
            }
        }
        _ => unreachable!(),
    }
}

/// Generate expression to convert target type directly to u32 (optimized path)
fn generate_type_to_u32(field_ty: &AstType, field_name: &syn::Ident, schema: &Schema) -> TokenStream {
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
                    unreachable!("Unexpected primitive in u32 write: {}", prim.name)
                }
            }
        },
        _ => unreachable!(),
    }
}

/// Generate read expression for a field from inline storage
fn generate_inline_read(
    field_ty: &AstType,
    schema: &Schema,
    byte_offset: usize,
    byte_size: usize,
    mode: &InlineStorageMode,
) -> TokenStream {
    // First, read the bytes from the appropriate location
    let read_bytes = generate_read_bytes(byte_offset, byte_size, mode);
    // Then, cast them to the appropriate type
    let cast_expr = generate_bytes_to_type(field_ty, schema, byte_size);

    quote! {
        #read_bytes
        #cast_expr
    }
}

/// Generate code to read bytes from inline storage
fn generate_read_bytes(byte_offset: usize, byte_size: usize, mode: &InlineStorageMode) -> TokenStream {
    let mut reads = TokenStream::new();

    match mode {
        InlineStorageMode::FourBytes => {
            // All data is in node.data.inline_data (u32)
            reads.extend(quote! {
                let u32_bytes = unsafe { node.data.inline_data }.to_le_bytes();
            });
            for i in 0..byte_size {
                let var_name = format_ident!("_b{}", i);
                let src_idx = byte_offset + i;
                reads.extend(quote! {
                    let #var_name = u32_bytes[#src_idx];
                });
            }
        }
        InlineStorageMode::Full => {
            // Bytes 0-3 in node.data.inline_data, bytes 4-6 in node.inline_data
            reads.extend(quote! {
                let u32_bytes = unsafe { node.data.inline_data }.to_le_bytes();
            });
            for i in 0..byte_size {
                let var_name = format_ident!("_b{}", i);
                let src_offset = byte_offset + i;
                let read_expr = if src_offset < INLINE_DATA_U32_SIZE {
                    quote!(u32_bytes[#src_offset])
                } else {
                    let arr_idx = src_offset - INLINE_DATA_U32_SIZE;
                    quote!(node.inline_data[#arr_idx])
                };
                reads.extend(quote! {
                    let #var_name = #read_expr;
                });
            }
        }
    }

    reads
}

/// Generate expression to convert bytes to the target type
fn generate_bytes_to_type(field_ty: &AstType, schema: &Schema, _byte_size: usize) -> TokenStream {
    match field_ty {
        AstType::Struct(_) | AstType::Enum(_) => {
            let field_inner_ty = field_ty.repr_ident(schema);
            quote! {
                let raw = u32::from_le_bytes([_b0, _b1, _b2, _b3]);
                unsafe { #field_inner_ty::from_node_id_unchecked(crate::NodeId::from_usize(raw as usize), ast) }
            }
        }
        AstType::Option(ast_option) => {
            let inner_ty = &schema.types[ast_option.inner_type_id];
            let field_inner_ident = inner_ty.repr_ident(schema);
            quote! {
                let raw = u32::from_le_bytes([_b0, _b1, _b2, _b3]);
                let opt = crate::OptionalNodeId::from_raw(raw);
                opt.map(|id| unsafe { #field_inner_ident::from_node_id_unchecked(id, ast) })
            }
        }
        AstType::Primitive(prim) => match prim.name {
            "bool" => quote! { _b0 != 0 },
            "u8" => quote! { _b0 },
            "i8" => quote! { _b0 as i8 },
            "u16" => quote! { u16::from_le_bytes([_b0, _b1]) },
            "i16" => quote! { i16::from_le_bytes([_b0, _b1]) },
            "u32" => quote! { u32::from_le_bytes([_b0, _b1, _b2, _b3]) },
            "i32" => quote! { i32::from_le_bytes([_b0, _b1, _b2, _b3]) },
            "BigIntId" => quote! {
                crate::BigIntId::from_usize(u32::from_le_bytes([_b0, _b1, _b2, _b3]) as usize)
            },
            // Small enums (1 byte)
            _ => {
                let prim_ty = format_ident!("{}", prim.name);
                quote! { #prim_ty::from_extra_data(_b0 as u64) }
            }
        },
        _ => unreachable!(),
    }
}

/// Generate write expression for a field to inline storage
fn generate_inline_write(
    field_ty: &AstType,
    field_name: &syn::Ident,
    byte_offset: usize,
    byte_size: usize,
    mode: &InlineStorageMode,
) -> TokenStream {
    // First, convert the value to bytes
    let to_bytes = generate_type_to_bytes(field_ty, field_name);
    // Then, write them to the appropriate location
    let write_bytes = generate_write_bytes(byte_offset, byte_size, mode);

    quote! {
        #to_bytes
        #write_bytes
    }
}

/// Generate expression to convert a field value to bytes
fn generate_type_to_bytes(field_ty: &AstType, field_name: &syn::Ident) -> TokenStream {
    match field_ty {
        AstType::Struct(_) | AstType::Enum(_) => {
            quote!(let field_bytes = (#field_name.node_id().index() as u32).to_le_bytes();)
        }
        AstType::Option(_) => {
            quote!(let field_bytes = crate::OptionalNodeId::from(#field_name.map(|n| n.node_id())).into_raw().to_le_bytes();)
        }
        AstType::Primitive(prim) => match prim.name {
            "bool" => quote!(let field_bytes = [#field_name as u8];),
            "u8" => quote!(let field_bytes = [#field_name];),
            "i8" => quote!(let field_bytes = [#field_name as u8];),
            "u16" | "i16" => quote!(let field_bytes = #field_name.to_le_bytes();),
            "u32" | "i32" => quote!(let field_bytes = #field_name.to_le_bytes();),
            "BigIntId" => quote!(let field_bytes = (#field_name.index() as u32).to_le_bytes();),
            // Small enums (1 byte)
            _ => quote!(let field_bytes = [(#field_name.to_extra_data() & 0xFF) as u8];),
        },
        _ => unreachable!(),
    }
}

/// Generate code to write bytes to inline storage
fn generate_write_bytes(byte_offset: usize, byte_size: usize, mode: &InlineStorageMode) -> TokenStream {
    match mode {
        InlineStorageMode::FourBytes => {
            // All data in node.data.inline_data (u32), need to read-modify-write
            let mut update_bytes = TokenStream::new();
            for i in 0..byte_size {
                let dst_idx = byte_offset + i;
                update_bytes.extend(quote! {
                    u32_bytes[#dst_idx] = field_bytes[#i];
                });
            }
            quote! {
                let mut u32_bytes = unsafe { node.data.inline_data }.to_le_bytes();
                #update_bytes
                node.data.inline_data = u32::from_le_bytes(u32_bytes);
            }
        }
        InlineStorageMode::Full => {
            // Bytes 0-3 in node.data.inline_data, bytes 4-6 in node.inline_data
            // Check if this field spans across the boundary
            let end_offset = byte_offset + byte_size;
            let in_u32_only = end_offset <= INLINE_DATA_U32_SIZE;
            let in_bytes_only = byte_offset >= INLINE_DATA_U32_SIZE;

            if in_u32_only {
                // Field is entirely in the u32
                let mut update_bytes = TokenStream::new();
                for i in 0..byte_size {
                    let dst_idx = byte_offset + i;
                    update_bytes.extend(quote! {
                        u32_bytes[#dst_idx] = field_bytes[#i];
                    });
                }
                quote! {
                    let mut u32_bytes = unsafe { node.data.inline_data }.to_le_bytes();
                    #update_bytes
                    node.data.inline_data = u32::from_le_bytes(u32_bytes);
                }
            } else if in_bytes_only {
                // Field is entirely in inline_data[0..3]
                let mut writes = TokenStream::new();
                for i in 0..byte_size {
                    let arr_idx = byte_offset - INLINE_DATA_U32_SIZE + i;
                    writes.extend(quote! {
                        node.inline_data[#arr_idx] = field_bytes[#i];
                    });
                }
                writes
            } else {
                // Field spans across u32 and inline_data (rare case)
                let mut update_u32 = TokenStream::new();
                let mut update_bytes = TokenStream::new();
                for i in 0..byte_size {
                    let src_offset = byte_offset + i;
                    if src_offset < INLINE_DATA_U32_SIZE {
                        update_u32.extend(quote! {
                            u32_bytes[#src_offset] = field_bytes[#i];
                        });
                    } else {
                        let arr_idx = src_offset - INLINE_DATA_U32_SIZE;
                        update_bytes.extend(quote! {
                            node.inline_data[#arr_idx] = field_bytes[#i];
                        });
                    }
                }
                quote! {
                    let mut u32_bytes = unsafe { node.data.inline_data }.to_le_bytes();
                    #update_u32
                    node.data.inline_data = u32::from_le_bytes(u32_bytes);
                    #update_bytes
                }
            }
        }
    }
}

/// Generate getters/setters for extra_data storage (original implementation)
fn generate_extra_data_property_accessors(
    ast: &AstStruct,
    schema: &Schema,
    field_getters: &mut TokenStream,
    field_setters: &mut TokenStream,
) {
    for (offset, field) in ast.fields.iter().enumerate() {
        let field_name = format_ident!("{}", field.name);
        let field_ty = &schema.types[field.type_id];

        let extra_data_name = map_field_type_to_extra_field(field_ty, schema);
        let (ret_ty, cast_expr) = match &field_ty {
            AstType::Vec(_) => (
                field_ty.repr_ident(schema),
                quote!(unsafe { ret.cast_to_typed() }),
            ),
            AstType::Option(ast) => {
                let option_field_ident = field_ty.repr_ident(schema);
                let cast_expr = match &schema.types[ast.inner_type_id] {
                    AstType::Vec(_) => {
                        quote!(unsafe { ret.cast_to_typed().to_option() })
                    }
                    _ => {
                        let field_inner_ident = schema.types[ast.inner_type_id].repr_ident(schema);
                        quote!( ret.map(|id| unsafe { #field_inner_ident::from_node_id_unchecked(id, ast) }) )
                    }
                };
                (option_field_ident, cast_expr)
            }
            AstType::Struct(_) | AstType::Enum(_) => {
                let field_inner_ty = field_ty.repr_ident(schema);
                (
                    field_inner_ty.clone(),
                    quote!( unsafe { #field_inner_ty::from_node_id_unchecked(ret, ast) }),
                )
            }
            _ if extra_data_name == "other" => {
                let field_ty = field_ty.repr_ident(schema);
                (field_ty.clone(), quote!(#field_ty::from_extra_data(ret)))
            }
            _ => (field_ty.repr_ident(schema), quote!(ret.into())),
        };
        let extra_data_name = format_ident!("{extra_data_name}");

        let getter_name = safe_ident(&field.name.to_case(Case::Snake));
        field_getters.extend(quote! {
            #[inline]
            pub fn #getter_name(&self, ast: &crate::Ast) -> #ret_ty {
                let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start } + #offset;

                debug_assert!(offset < ast.extra_data.len());
                let ret = unsafe { ast.extra_data.as_raw_slice().get_unchecked(offset.index()).#extra_data_name };
                #cast_expr
            }
        });

        let extra_data_value = match &field_ty {
            AstType::Option(ast_option) => {
                let inner_ty = &schema.types[ast_option.inner_type_id];
                match inner_ty {
                    AstType::Vec(_) => quote!(#field_name.map(|n| n.inner).into()),
                    _ => quote!(#field_name.map(|n| n.node_id()).into()),
                }
            }
            AstType::Struct(_) | AstType::Enum(_) => quote!(#field_name.node_id().into()),
            _ if extra_data_name == "other" => {
                quote!(#field_name.to_extra_data())
            }
            _ => quote!(#field_name.into()),
        };
        let setter_name = format_ident!("set_{}", field_name);
        field_setters.extend(quote! {
            #[inline]
            pub fn #setter_name(&self, ast: &mut crate::Ast, #field_name: #ret_ty) {
                let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start } + #offset;

                debug_assert!(offset < ast.extra_data.len());
                unsafe { ast.extra_data.as_raw_slice_mut().get_unchecked_mut(offset.index()).#extra_data_name = #extra_data_value };
            }
        });
    }
}

fn generate_property_for_enum(ast: &AstEnum, schema: &Schema) -> TokenStream {
    let name = format_ident!("{}", ast.name);

    let mut field_getters = TokenStream::new();
    let mut field_setters = TokenStream::new();
    let mut is_variant = TokenStream::new();
    let mut as_variant = TokenStream::new();

    let mut get_span_arms = TokenStream::new();
    let mut set_span_arms = TokenStream::new();
    for variant in ast.variants.iter() {
        let variant_name = format_ident!("{}", variant.name);
        get_span_arms.extend(quote! {
            Self::#variant_name(it) => it.span(ast),
        });
        set_span_arms.extend(quote! {
            Self::#variant_name(it) => it.set_span(ast, span),
        });

        let is_fn_name = format_ident!("is_{}", variant.name.to_case(Case::Snake));
        is_variant.extend(quote! {
            #[inline]
            pub fn #is_fn_name(&self) -> bool {
                matches!(self, Self::#variant_name(_))
            }
        });

        let as_fn_name = format_ident!("as_{}", variant.name.to_case(Case::Snake));
        let struct_name = format_ident!("{}", schema.types[variant.type_id.unwrap()].name());
        as_variant.extend(quote! {
            #[inline]
            pub fn #as_fn_name(&self) -> Option<&#struct_name> {
                match self {
                    Self::#variant_name(it) => Some(it),
                    _ => None,
                }
            }
        });
    }

    field_getters.extend(quote! {
        #[inline]
        pub fn span(&self, ast: &crate::Ast) -> crate::Span {
            match self {
                #get_span_arms
            }
        }
        #[inline]
        pub fn span_lo(&self, ast: &crate::Ast) -> crate::BytePos {
            self.span(ast).lo
        }
        #[inline]
        pub fn span_hi(&self, ast: &crate::Ast) -> crate::BytePos {
            self.span(ast).hi
        }
    });

    field_setters.extend(quote! {
        #[inline]
        pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
            match self {
                #set_span_arms
            }
        }
    });

    quote! {
        impl #name {
            #field_getters
            #field_setters
            #is_variant
            #as_variant
        }
    }
}
