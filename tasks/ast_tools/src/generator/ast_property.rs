use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    AST_CRATE_PATH,
    output::{RawOutput, RustOutput, output_path},
    schema::{AstEnum, AstStruct, AstType, Schema},
    util::{map_field_type_to_extra_field, safe_ident},
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
            ast.nodes[self.0].span
        }
    });

    field_setters.extend(quote! {
        #[inline]
        pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
            ast.nodes[self.0].span = span;
        }
    });

    for (offset, field) in ast.fields.iter().enumerate() {
        let field_name = format_ident!("{}", field.name);
        let field_ty = &schema.types[field.type_id];

        let (ret_ty, cast_expr) = match &field_ty {
            AstType::Vec(_) => (
                field_ty.repr_ident(schema),
                quote!(unsafe { ret.cast_to_typed() }),
            ),
            AstType::Option(ast) => {
                let option_field_ident = field_ty.repr_ident(schema);
                let field_inner_ident = schema.types[ast.inner_type_id].repr_ident(schema);
                (
                    option_field_ident,
                    quote!( ret.map(|id| #field_inner_ident::from_node_id(id, ast)) ),
                )
            }
            AstType::Struct(_) | AstType::Enum(_) => {
                let field_inner_ty = field_ty.repr_ident(schema);
                (
                    field_inner_ty.clone(),
                    quote!( #field_inner_ty::from_node_id(ret, ast) ),
                )
            }
            _ => (field_ty.repr_ident(schema), quote!(ret.into())),
        };
        let extra_data_name = format_ident!("{}", map_field_type_to_extra_field(field_ty));

        let getter_name = safe_ident(&field.name.to_case(Case::Snake));
        field_getters.extend(quote! {
            #[inline]
            pub fn #getter_name(&self, ast: &crate::Ast) -> #ret_ty {
                let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + #offset;
                let ret = unsafe { ast.extra_data[offset].#extra_data_name };
                #cast_expr
            }
        });

        let extra_data_value = match &field_ty {
            AstType::Option(_) => {
                quote!(#field_name.optional_node_id().into())
            }
            AstType::Struct(_) | AstType::Enum(_) => quote!(#field_name.node_id().into()),
            _ => quote!(#field_name.into()),
        };
        let setter_name = format_ident!("set_{}", field_name);
        field_setters.extend(quote! {
            #[inline]
            pub fn #setter_name(&self, ast: &mut crate::Ast, #field_name: #ret_ty) {
                let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + #offset;
                ast.extra_data[offset].#extra_data_name = #extra_data_value;
            }
        });
    }

    quote! {
        impl #name {
            #field_getters
            #field_setters
        }
    }
}

fn generate_property_for_enum(_ast: &AstEnum, _schema: &Schema) -> TokenStream {
    TokenStream::new()
}
