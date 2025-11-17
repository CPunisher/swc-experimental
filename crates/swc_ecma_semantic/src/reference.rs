use crate::symbol::SymbolId;

oxc_index::define_index_type! {
    pub struct ReferenceId = u32;
}

pub struct Reference {
    symbol_id: SymbolId,
}
