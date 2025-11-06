use std::collections::BTreeMap;

use phf::{Set as PhfSet, phf_set};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::schema::{self, AstEnum, AstType, Schema};

/// Reserved word in Rust.
/// From <https://doc.rust-lang.org/reference/keywords.html>.
pub(crate) static RESERVED_NAMES: PhfSet<&'static str> = phf_set! {
    // Strict keywords
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for", "if",
    "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self",
    "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where", "while", "async",
    "await", "dyn",
    // Reserved keywords
    "abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof", "unsized",
    "virtual", "yield", "try",
    // Weak keywords
    "macro_rules", "union", // "dyn" also listed as a weak keyword, but is already on strict list
};

/// Returns `true` if `name` is a reserved word in Rust.
pub fn is_reserved_name(name: &str) -> bool {
    RESERVED_NAMES.contains(name)
}

pub fn safe_ident(name: &str) -> Ident {
    if is_reserved_name(name) {
        format_ident!("{name}_")
    } else {
        format_ident!("{name}")
    }
}

pub fn map_field_type_to_extra_field(ast: &AstType) -> &str {
    match ast {
        AstType::Struct(_) | AstType::Enum(_) => "node",
        AstType::Vec(_) => "sub_range",
        AstType::Option(_) => "optional_node",
        AstType::Primitive(ast_primitive) => match ast_primitive.name {
            "Span" => "span",
            "AtomRef" => "atom",
            "OptionalAtomRef" => "optional_atom",
            "BigIntId" => "bigint",
            "bool" => "bool",
            "f64" => "number",
            _ => "other",
        },
    }
}

pub struct FlattenElement {
    /// Final nested struct ident, corresponding to NodeKind::XXX
    pub kind: Ident,
    /// From the first enum::variant to the last enum::variant
    pub path: Vec<TokenStream>,
}

pub fn flat_enum_type(ast: &AstEnum, schema: &Schema) -> BTreeMap<String, FlattenElement> {
    let mut flatten = BTreeMap::new();
    let mut worklist = Vec::new();

    let enum_ident = format_ident!("{}", ast.name);
    for variant in ast.variants.iter() {
        let variant_ident = format_ident!("{}", variant.name);
        let path = quote!( #enum_ident::#variant_ident );
        worklist.push((variant, vec![path]));
    }

    while let Some((variant, path)) = worklist.pop() {
        let Some(variant_inner_ty_id) = variant.type_id else {
            continue;
        };

        let variant_inner_ty = &schema.types[variant_inner_ty_id];
        match variant_inner_ty {
            schema::AstType::Struct(ast_struct) => {
                let kind = format_ident!("{}", ast_struct.name);
                flatten.insert(ast_struct.name.clone(), FlattenElement { kind, path });
            }
            schema::AstType::Enum(ast_enum) => {
                let nested_enum_ident = format_ident!("{}", ast_enum.name);
                for nested_variant in ast_enum.variants.iter() {
                    let nested_variant_ident = format_ident!("{}", nested_variant.name);
                    let mut path = path.clone();
                    path.push(quote! ( #nested_enum_ident::#nested_variant_ident ));
                    worklist.push((nested_variant, path));
                }
            }
            _ => unreachable!(),
        }
    }

    flatten
}
