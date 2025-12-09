use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    AST_CRATE_PATH,
    output::{RawOutput, RustOutput, output_path},
    schema::{AstType, Schema},
    util::map_field_type_to_extra_field,
};

pub fn ast_visitor(schema: &Schema) -> RawOutput {
    let mut visit_functions = TokenStream::new();
    let mut visit_with_impls = TokenStream::new();
    let mut visit_mut_functions = TokenStream::new();
    let mut visit_mut_with_impls = TokenStream::new();

    visit_functions.extend(quote! {
        fn ast(&self) -> &Ast;

        #[inline]
        fn enter_node(&mut self, node_id: NodeId) {}
        #[inline]
        fn leave_node(&mut self, node_id: NodeId) {}
    });

    visit_mut_functions.extend(quote! {
        fn ast(&mut self) -> &mut Ast;

        #[inline]
        fn enter_node(&mut self, node_id: NodeId) {}
        #[inline]
        fn leave_node(&mut self, node_id: NodeId) {}
    });

    for ty in schema.types.iter() {
        match ty {
            AstType::Struct(ast) => {
                let fn_name = format_ident!("visit_{}", &ast.name.to_case(Case::Snake));
                let fn_mut_name = format_ident!("visit_mut_{}", &ast.name.to_case(Case::Snake));
                let ty_ident = ty.repr_ident(schema);

                // Visit/VisitMut
                visit_functions.extend(quote! {
                    #[inline]
                    fn #fn_name(&mut self, node: #ty_ident) {
                        self.enter_node(node.node_id());
                        <#ty_ident as VisitWith<Self>>::visit_children_with(node, self);
                        self.leave_node(node.node_id());
                    }
                });
                visit_mut_functions.extend(quote! {
                    #[inline]
                    fn #fn_mut_name(&mut self, node: #ty_ident) {
                        self.enter_node(node.node_id());
                        <#ty_ident as VisitMutWith<Self>>::visit_mut_children_with(node, self);
                        self.leave_node(node.node_id());
                    }
                });

                // VisitWith/VisitMutWith
                let mut visit_children = TokenStream::new();
                let mut visit_mut_children = TokenStream::new();

                for (offset, field) in ast.fields.iter().enumerate() {
                    let field_ty = &schema.types[field.type_id];
                    let field_ty_ident = field_ty.repr_ident(schema);

                    let extra_data_name = map_field_type_to_extra_field(field_ty);
                    let extra_data_name = format_ident!("{extra_data_name}");
                    let cast_expr = match &field_ty {
                        AstType::Vec(_) => quote!(unsafe { ret.cast_to_typed() }),
                        AstType::Option(ast) => {
                            let field_inner_ident =
                                schema.types[ast.inner_type_id].repr_ident(schema);
                            quote!( ret.map(|id| unsafe { #field_inner_ident::from_node_id_unchecked(id, visitor.ast()) }) )
                        }
                        AstType::Struct(_) | AstType::Enum(_) => {
                            let field_inner_ty = field_ty.repr_ident(schema);
                            quote!( unsafe { #field_inner_ty::from_node_id_unchecked(ret, visitor.ast()) })
                        }
                        _ if extra_data_name == "other" => {
                            let field_ty = field_ty.repr_ident(schema);
                            quote!(#field_ty::from_extra_data(ret))
                        }
                        _ => quote!(ret.into()),
                    };
                    visit_children.extend(quote! {
                        let ret = unsafe { visitor.ast().extra_data.as_raw_slice().get_unchecked((offset + #offset).index()).#extra_data_name };
                        <#field_ty_ident as VisitWith<V>>::visit_with(#cast_expr, visitor);
                    });
                    visit_mut_children.extend(quote! {
                        let ret = unsafe { visitor.ast().extra_data.as_raw_slice().get_unchecked((offset + #offset).index()).#extra_data_name };
                        <#field_ty_ident as VisitMutWith<V>>::visit_mut_with(#cast_expr, visitor);
                    });
                }

                visit_with_impls.extend(quote! {
                    impl<V: ?Sized + Visit> VisitWith<V> for #ty_ident {
                        fn visit_with(self, visitor: &mut V) {
                            <V as Visit>::#fn_name(visitor, self)
                        }

                        fn visit_children_with(self, visitor: &mut V) {
                            let offset = unsafe { visitor.ast().nodes.get_unchecked(self.0).data.extra_data_start };
                            #visit_children
                        }
                    }
                });
                visit_mut_with_impls.extend(quote! {
                    impl<V: ?Sized + VisitMut> VisitMutWith<V> for #ty_ident {
                        fn visit_mut_with(self, visitor: &mut V) {
                            <V as VisitMut>::#fn_mut_name(visitor, self)
                        }

                        fn visit_mut_children_with(self, visitor: &mut V) {
                            let offset = unsafe { visitor.ast().nodes.get_unchecked(self.0).data.extra_data_start };
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
                    fn #fn_name(&mut self, node: #ty_ident) {
                        <#ty_ident as VisitWith<Self>>::visit_children_with(node, self)
                    }
                });
                visit_mut_functions.extend(quote! {
                    #[inline]
                    fn #fn_mut_name(&mut self, node: #ty_ident) {
                        <#ty_ident as VisitMutWith<Self>>::visit_mut_children_with(node, self)
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
                        Self::#variant_name(it) => <#variant_ty_ident as VisitWith<V>>::visit_with(it, visitor),
                    });
                    visit_mut_children_arms.extend(quote! {
                        Self::#variant_name(it) => <#variant_ty_ident as VisitMutWith<V>>::visit_mut_with(it, visitor),
                    });
                }

                visit_with_impls.extend(quote! {
                    impl<V: ?Sized + Visit> VisitWith<V> for #ty_ident {
                        fn visit_with(self, visitor: &mut V) {
                            <V as Visit>::#fn_name(visitor, self)
                        }

                        fn visit_children_with(self, visitor: &mut V) {
                            match self {
                                #visit_children_arms
                            }
                        }
                    }
                });
                visit_mut_with_impls.extend(quote! {
                    impl<V: ?Sized + VisitMut> VisitMutWith<V> for #ty_ident {
                        fn visit_mut_with(self, visitor: &mut V) {
                            <V as VisitMut>::#fn_mut_name(visitor, self)
                        }

                        fn visit_mut_children_with(self, visitor: &mut V) {
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
                    fn #fn_name(&mut self, node: #ty_ident) {
                        <#ty_ident as VisitWith<Self>>::visit_children_with(node, self);
                    }
                });
                visit_mut_functions.extend(quote! {
                    #[inline]
                    fn #fn_mut_name(&mut self, node: #ty_ident) {
                        <#ty_ident as VisitMutWith<Self>>::visit_mut_children_with(node, self);
                    }
                });

                // VisitWith/VisitMutWith
                visit_with_impls.extend(quote! {
                    impl<V: ?Sized + Visit> VisitWith<V> for #ty_ident {
                        fn visit_with(self, visitor: &mut V) {
                            <V as Visit>::#fn_name(visitor, self)
                        }

                        fn visit_children_with(self, visitor: &mut V) {
                            match self {
                                Some(it) => it.visit_with(visitor),
                                None => {}
                            }
                        }
                    }
                });
                visit_mut_with_impls.extend(quote! {
                    impl<V: ?Sized + VisitMut> VisitMutWith<V> for #ty_ident {
                        fn visit_mut_with(self, visitor: &mut V) {
                            <V as VisitMut>::#fn_mut_name(visitor, self)
                        }

                        fn visit_mut_children_with(self, visitor: &mut V) {
                            match self {
                                Some(it) => it.visit_mut_with(visitor),
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
                    fn #fn_name(&mut self, node: #ty_ident) {
                        <#ty_ident as VisitWith<Self>>::visit_children_with(node, self)
                    }
                });
                visit_mut_functions.extend(quote! {
                    #[inline]
                    fn #fn_mut_name(&mut self, node: #ty_ident) {
                        <#ty_ident as VisitMutWith<Self>>::visit_mut_children_with(node, self)
                    }
                });

                // VisitWith/VisitMutWith
                let get_node = match inner_type {
                    AstType::Option(_) => {
                        quote!( let child = visitor.ast().get_opt_node_in_sub_range(child); )
                    }
                    _ => quote! ( let child = visitor.ast().get_node_in_sub_range(child); ),
                };
                visit_with_impls.extend(quote! {
                    impl<V: ?Sized + Visit> VisitWith<V> for #ty_ident {
                        fn visit_with(self, visitor: &mut V) {
                            <V as Visit>::#fn_name(visitor, self)
                        }

                        fn visit_children_with(self, visitor: &mut V) {
                            for child in self.iter() {
                                #get_node
                                child.visit_with(visitor);
                            }
                        }
                    }
                });
                visit_mut_with_impls.extend(quote! {
                    impl<V: ?Sized + VisitMut> VisitMutWith<V> for #ty_ident {
                        fn visit_mut_with(self, visitor: &mut V) {
                            <V as VisitMut>::#fn_mut_name(visitor, self)
                        }

                        fn visit_mut_children_with(self, visitor: &mut V) {
                            for child in self.iter() {
                                #get_node
                                child.visit_mut_with(visitor);
                            }
                        }
                    }
                });
            }
            _ => continue,
        };
    }

    let output = quote! {
            #![allow(unused, clippy::useless_conversion, clippy::single_match)]
            use crate::{Ast, ast::*, node_id::*};
            use swc_core::common::Span;

            pub trait Visit {
                #visit_functions
            }

            pub trait VisitWith<V: ?Sized + Visit> {
                fn visit_with(self, visitor: &mut V);
                fn visit_children_with(self, visitor: &mut V);
            }

            #visit_with_impls

            pub trait VisitMut {
                #visit_mut_functions
            }

            pub trait VisitMutWith<V: ?Sized + VisitMut> {
                fn visit_mut_with(self, visitor: &mut V);
                fn visit_mut_children_with(self, visitor: &mut V);
            }

            #visit_mut_with_impls
    };

    RustOutput {
        path: output_path(AST_CRATE_PATH, "ast_visitor"),
        tokens: output,
    }
    .into()
}
