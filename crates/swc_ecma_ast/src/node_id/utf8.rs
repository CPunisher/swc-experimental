use string_interner::symbol::SymbolU32;

/// A reference to a utf8 string in the string allocator.
#[derive(Debug, Clone, Copy)]
pub struct Utf8Ref(pub(crate) SymbolU32);

impl Utf8Ref {
    pub(crate) const fn new(symbol: SymbolU32) -> Self {
        Self(symbol)
    }
}

pub type OptionalUtf8Ref = Option<Utf8Ref>;
