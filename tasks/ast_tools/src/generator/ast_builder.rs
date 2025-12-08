use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};

use crate::{
    AST_CRATE_PATH,
    output::{RawOutput, RustOutput, output_path},
    schema::{AstEnum, AstStruct, AstType, Schema},
    util::{map_field_type_to_extra_field, safe_ident},
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

    let mut add_extra_data = TokenStream::new();
    for (index, field) in ast.fields.iter().enumerate() {
        let extra_data_id = format_ident!("_f{}", index);
        let field_name = format_ident!("{}", field.name);

        let field_ty = &schema.types[field.type_id];
        let extra_data_field = map_field_type_to_extra_field(field_ty);
        let field_value = match field_ty {
            AstType::Option(_) => {
                quote!( #field_name.optional_node_id() )
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
