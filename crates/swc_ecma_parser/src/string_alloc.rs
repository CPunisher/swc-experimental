use oxc_index::IndexVec;
use swc_atoms::wtf8::{Wtf8, Wtf8Buf};
use swc_experimental_ecma_ast::{AtomId, Wtf8AtomId};

#[derive(Clone)]
pub struct StringAllocator {
    allocated_utf8: IndexVec<AtomId, String>,
    allocated_wtf8: IndexVec<Wtf8AtomId, Wtf8Buf>,
}

impl StringAllocator {
    pub fn new() -> Self {
        Self {
            allocated_utf8: IndexVec::new(),
            allocated_wtf8: IndexVec::new(),
        }
    }

    pub fn alloc_utf8(&mut self, utf8: String) -> AtomId {
        self.allocated_utf8.push(utf8)
    }

    pub fn alloc_wtf8(&mut self, wtf8: Wtf8Buf) -> Wtf8AtomId {
        self.allocated_wtf8.push(wtf8)
    }

    pub fn get_utf8(&self, id: AtomId) -> &str {
        &self.allocated_utf8[id]
    }

    pub fn get_wtf8(&self, id: Wtf8AtomId) -> &Wtf8 {
        &self.allocated_wtf8[id]
    }
}
