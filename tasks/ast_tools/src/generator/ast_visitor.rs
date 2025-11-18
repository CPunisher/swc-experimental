use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    VISIT_CRATE_PATH,
    output::{RawOutput, RustOutput, output_path},
    schema::{AstType, Schema},
};

pub fn ast_visitor(schema: &Schema) -> RawOutput {
    let mut visit_functions = TokenStream::new();
    let mut visit_with_impls = TokenStream::new();
    let mut visit_mut_functions = TokenStream::new();
    let mut visit_mut_with_impls = TokenStream::new();

    for ty in schema.types.iter() {
        match ty {
            AstType::Struct(ast) => {
                let fn_name = format_ident!("visit_{}", &ast.name.to_case(Case::Snake));
                let fn_mut_name = format_ident!("visit_mut_{}", &ast.name.to_case(Case::Snake));
                let ty_ident = ty.repr_ident(schema);

                // Visit/VisitMut
                visit_functions.extend(quote! {
                    #[inline]
                    fn #fn_name(&mut self, node: #ty_ident, ast: &Ast) {
                        <#ty_ident as VisitWith<Self>>::visit_children_with(node, self, ast)
                    }
                });
                visit_mut_functions.extend(quote! {
                    #[inline]
                    fn #fn_mut_name(&mut self, node: #ty_ident, ast: &mut Ast) {
                        <#ty_ident as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
                    }
                });

                // VisitWith/VisitMutWith
                let mut visit_children = TokenStream::new();
                let mut visit_mut_children = TokenStream::new();

                for field in ast.fields.iter() {
                    let field_ty = &schema.types[field.type_id];
                    let field_ty_ident = field_ty.repr_ident(schema);
                    let field_accessor = format_ident!("{}", field.name.to_case(Case::Snake));
                    visit_children.extend(quote! {
                        <#field_ty_ident as VisitWith<V>>::visit_with(self.#field_accessor(ast), visitor, ast);
                    });
                    visit_mut_children.extend(quote! {
                        <#field_ty_ident as VisitMutWith<V>>::visit_mut_with(self.#field_accessor(ast), visitor, ast);
                    });
                }

                visit_with_impls.extend(quote! {
                    impl<V: ?Sized + Visit> VisitWith<V> for #ty_ident {
                        fn visit_with(self, visitor: &mut V, ast: &Ast) {
                            <V as Visit>::#fn_name(visitor, self, ast)
                        }

                        fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
                            #visit_children
                        }
                    }
                });
                visit_mut_with_impls.extend(quote! {
                    impl<V: ?Sized + VisitMut> VisitMutWith<V> for #ty_ident {
                        fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
                            <V as VisitMut>::#fn_mut_name(visitor, self, ast)
                        }

                        fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
                            #visit_mut_children
                        }
                    }
                });
            }
            AstType::Enum(ast) => {
                let fn_name = format_ident!("visit_{}", &ast.name.to_case(Case::Snake));
                let fn_mut_name = format_ident!("visit_mut_{}", &ast.name.to_case(Case::Snake));
                let ty_ident = ty.repr_ident(schema);

                // Visit/VisitMut
                visit_functions.extend(quote! {
                    #[inline]
                    fn #fn_name(&mut self, node: #ty_ident, ast: &Ast) {
                        <#ty_ident as VisitWith<Self>>::visit_children_with(node, self, ast)
                    }
                });
                visit_mut_functions.extend(quote! {
                    #[inline]
                    fn #fn_mut_name(&mut self, node: #ty_ident, ast: &mut Ast) {
                        <#ty_ident as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
                    }
                });

                // VisitWith/VisitMutWith
                let mut visit_children_arms = TokenStream::new();
                let mut visit_mut_children_arms = TokenStream::new();

                for variant in ast.variants.iter() {
                    let Some(variant_type_id) = variant.type_id else {
                        continue;
                    };

                    let variant_ty = &schema.types[variant_type_id];
                    let variant_ty_ident = variant_ty.repr_ident(schema);
                    let variant_name = format_ident!("{}", variant.name);
                    visit_children_arms.extend(quote! {
                        Self::#variant_name(it) => <#variant_ty_ident as VisitWith<V>>::visit_with(it, visitor, ast),
                    });
                    visit_mut_children_arms.extend(quote! {
                        Self::#variant_name(it) => <#variant_ty_ident as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
                    });
                }

                visit_with_impls.extend(quote! {
                    impl<V: ?Sized + Visit> VisitWith<V> for #ty_ident {
                        fn visit_with(self, visitor: &mut V, ast: &Ast) {
                            <V as Visit>::#fn_name(visitor, self, ast)
                        }

                        fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
                            match self {
                                #visit_children_arms
                            }
                        }
                    }
                });
                visit_mut_with_impls.extend(quote! {
                    impl<V: ?Sized + VisitMut> VisitMutWith<V> for #ty_ident {
                        fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
                            <V as VisitMut>::#fn_mut_name(visitor, self, ast)
                        }

                        fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
                            match self {
                                #visit_mut_children_arms
                            }
                        }
                    }
                });
            }
            AstType::Option(ast) => {
                let inner_type = &schema.types[ast.inner_type_id];
                let fn_name =
                    format_ident!("visit_opt_{}", &inner_type.name().to_case(Case::Snake));
                let fn_mut_name =
                    format_ident!("visit_mut_opt_{}", &inner_type.name().to_case(Case::Snake));
                let ty_ident = ty.repr_ident(schema);

                // Visit/VisitMut
                visit_functions.extend(quote! {
                    #[inline]
                    fn #fn_name(&mut self, node: #ty_ident, ast: &Ast) {
                        <#ty_ident as VisitWith<Self>>::visit_children_with(node, self, ast)
                    }
                });
                visit_mut_functions.extend(quote! {
                    #[inline]
                    fn #fn_mut_name(&mut self, node: #ty_ident, ast: &mut Ast) {
                        <#ty_ident as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
                    }
                });

                // VisitWith/VisitMutWith
                visit_with_impls.extend(quote! {
                    impl<V: ?Sized + Visit> VisitWith<V> for #ty_ident {
                        fn visit_with(self, visitor: &mut V, ast: &Ast) {
                            <V as Visit>::#fn_name(visitor, self, ast)
                        }

                        fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
                            match self {
                                Some(it) => it.visit_with(visitor, ast),
                                None => {}
                            }
                        }
                    }
                });
                visit_mut_with_impls.extend(quote! {
                    impl<V: ?Sized + VisitMut> VisitMutWith<V> for #ty_ident {
                        fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
                            <V as VisitMut>::#fn_mut_name(visitor, self, ast)
                        }

                        fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
                            match self {
                                Some(it) => it.visit_mut_with(visitor, ast),
                                None => {}
                            }
                        }
                    }
                });
            }
            AstType::Vec(ast) => {
                let inner_type = &schema.types[ast.inner_type_id];
                let (fn_name, fn_mut_name) = match inner_type {
                    AstType::Option(opt) => {
                        let inner_type = &schema.types[opt.inner_type_id];
                        let fn_name =
                            format_ident!("visit_opt_{}s", &inner_type.name().to_case(Case::Snake));
                        let fn_mut_name = format_ident!(
                            "visit_mut_opt_{}s",
                            &inner_type.name().to_case(Case::Snake)
                        );
                        (fn_name, fn_mut_name)
                    }
                    _ => {
                        let fn_name =
                            format_ident!("visit_{}s", &inner_type.name().to_case(Case::Snake));
                        let fn_mut_name =
                            format_ident!("visit_mut_{}s", &inner_type.name().to_case(Case::Snake));
                        (fn_name, fn_mut_name)
                    }
                };
                let ty_ident = ty.repr_ident(schema);

                // Visit/VisitMut
                visit_functions.extend(quote! {
                    #[inline]
                    fn #fn_name(&mut self, node: #ty_ident, ast: &Ast) {
                        <#ty_ident as VisitWith<Self>>::visit_children_with(node, self, ast)
                    }
                });
                visit_mut_functions.extend(quote! {
                    #[inline]
                    fn #fn_mut_name(&mut self, node: #ty_ident, ast: &mut Ast) {
                        <#ty_ident as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
                    }
                });

                // VisitWith/VisitMutWith
                let get_node = match inner_type {
                    AstType::Option(_) => quote!( let child = ast.get_opt_node(child); ),
                    _ => quote! ( let child = ast.get_node(child); ),
                };
                visit_with_impls.extend(quote! {
                    impl<V: ?Sized + Visit> VisitWith<V> for #ty_ident {
                        fn visit_with(self, visitor: &mut V, ast: &Ast) {
                            <V as Visit>::#fn_name(visitor, self, ast)
                        }

                        fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
                            for child in self.iter() {
                                #get_node
                                child.visit_with(visitor, ast);
                            }
                        }
                    }
                });
                visit_mut_with_impls.extend(quote! {
                    impl<V: ?Sized + VisitMut> VisitMutWith<V> for #ty_ident {
                        fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
                            <V as VisitMut>::#fn_mut_name(visitor, self, ast)
                        }

                        fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
                            for child in self.iter() {
                                #get_node
                                child.visit_mut_with(visitor, ast);
                            }
                        }
                    }
                });
            }
            _ => continue,
        };
    }

    let output = quote! {
            #![allow(unused)]
            use swc_common::Span;

            use swc_experimental_ecma_ast::*;

            pub trait Visit {
                #visit_functions
            }

            pub trait VisitWith<V: ?Sized + Visit> {
                fn visit_with(self, visitor: &mut V, ast: &Ast);
                fn visit_children_with(self, visitor: &mut V, ast: &Ast);
            }

            #visit_with_impls

            pub trait VisitMut {
                #visit_mut_functions
            }

            pub trait VisitMutWith<V: ?Sized + VisitMut> {
                fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast);
                fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast);
            }

            #visit_mut_with_impls
    };

    RustOutput {
        path: output_path(VISIT_CRATE_PATH, "ast_visitor"),
        tokens: output,
    }
    .into()
}
