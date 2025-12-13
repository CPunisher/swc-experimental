use std::collections::BTreeMap;

use phf::{Set as PhfSet, phf_set};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::schema::{self, AstEnum, AstStruct, AstType, Schema};

/// Maximum bytes for inline storage
pub const INLINE_DATA_U32_SIZE: usize = 4; // NodeData.inline_data (u32)
pub const INLINE_DATA_BYTES_SIZE: usize = 3; // AstNode.inline_data ([u8; 3])
pub const INLINE_TOTAL_SIZE: usize = INLINE_DATA_U32_SIZE + INLINE_DATA_BYTES_SIZE; // 7 bytes

/// Inline storage mode for a struct
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InlineStorageMode {
    /// Total size ≤4 bytes: all fields stored in NodeData.inline_data (u32)
    FourBytes,
    /// Total size >4 and ≤7 bytes: use NodeData.inline_data + AstNode.inline_data
    Full,
}

/// Get the byte size for a field type, or None if not inlinable
pub fn field_byte_size(ty: &AstType, schema: &Schema) -> Option<usize> {
    match ty {
        // NodeId types are 4 bytes (u32)
        AstType::Struct(_) | AstType::Enum(_) => Some(4),
        // Vec requires SubRange (8 bytes) - too big for inline
        AstType::Vec(_) => None,
        // Option<NodeId> is 4 bytes (OptionalNodeId)
        AstType::Option(ast_option) => {
            let inner_ty = &schema.types[ast_option.inner_type_id];
            match inner_ty {
                // Option<Vec<T>> needs OptionalSubRange - too big
                AstType::Vec(_) => None,
                // Option<Struct/Enum> is OptionalNodeId (4 bytes)
                _ => Some(4),
            }
        }
        AstType::Primitive(prim) => match prim.name {
            "bool" => Some(1),
            "u8" | "i8" => Some(1),
            "u16" | "i16" => Some(2),
            "u32" | "i32" => Some(4),
            "BigIntId" => Some(4),
            // Small enums (1 byte)
            "UnaryOp" | "BinaryOp" | "AssignOp" | "UpdateOp" | "MetaPropKind" | "MethodKind" => {
                Some(1)
            }
            "VarDeclKind" | "ImportPhase" => Some(1),
            // These are too big for inline storage
            _ => None,
        },
    }
}

/// Layout information for inline storage
#[derive(Debug, Clone)]
pub struct InlineLayout {
    /// Storage mode: FourBytes or Full
    pub mode: InlineStorageMode,
    /// Fields with their byte offsets within the 7-byte inline space
    /// Offset 0-3: NodeData.inline_data (u32)
    /// Offset 4-6: AstNode.inline_data ([u8; 3])
    pub fields: Vec<(usize, usize, usize)>, // (field_index, byte_offset, byte_size)
}

/// Calculate the inline layout for a struct
/// Returns None if the struct cannot be inlined
pub fn calculate_inline_layout(ast: &AstStruct, schema: &Schema) -> Option<InlineLayout> {
    let mut fields: Vec<(usize, usize, usize)> = Vec::new();
    let mut current_offset = 0usize;

    for (index, field) in ast.fields.iter().enumerate() {
        let field_ty = &schema.types[field.type_id];
        let size = field_byte_size(field_ty, schema)?;

        if current_offset + size > INLINE_TOTAL_SIZE {
            return None; // Exceeds 7 bytes
        }

        fields.push((index, current_offset, size));
        current_offset += size;
    }

    let mode = if current_offset <= INLINE_DATA_U32_SIZE {
        InlineStorageMode::FourBytes
    } else {
        InlineStorageMode::Full
    };

    Some(InlineLayout { mode, fields })
}

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

pub fn map_field_type_to_extra_field<'a>(ast: &'a AstType, schema: &'a Schema) -> &'a str {
    match ast {
        AstType::Struct(_) | AstType::Enum(_) => "node",
        AstType::Vec(_) => "sub_range",
        AstType::Option(ast_option) => {
            let inner_ty = &schema.types[ast_option.inner_type_id];
            match inner_ty {
                AstType::Vec(_) => "optional_sub_range",
                _ => "optional_node",
            }
        }
        AstType::Primitive(ast_primitive) => match ast_primitive.name {
            "Span" => "span",
            "Utf8Ref" => "utf8",
            "Wtf8Ref" => "wtf8",
            "OptionalUtf8Ref" => "optional_utf8",
            "OptionalWtf8Ref" => "optional_wtf8",
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
