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

    for (offset, field) in ast.fields.iter().enumerate() {
        let field_name = format_ident!("{}", field.name);
        let field_ty = &schema.types[field.type_id];

        let extra_data_name = map_field_type_to_extra_field(field_ty);
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
                    quote!( ret.map(|id| unsafe { #field_inner_ident::from_node_id_unchecked(id, ast) }) ),
                )
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
            AstType::Option(_) => {
                quote!(#field_name.optional_node_id().into())
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

    quote! {
        impl #name {
            #field_getters
            #field_setters
        }
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
