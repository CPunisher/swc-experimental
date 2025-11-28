use oxc_index::IndexVec;
use swc_atoms::wtf8::{CodePoint, Wtf8, Wtf8Buf};
use swc_experimental_ecma_ast::{AtomId, Wtf8AtomId};

#[derive(Clone)]
pub struct StringAllocator {
    allocated_utf8: String,
    allocated_wtf8: Wtf8Buf,
    utf8_mapping: IndexVec<AtomId, AllocatedUtf8>,
    wtf8_mapping: IndexVec<Wtf8AtomId, AllocatedWtf8>,
}

impl StringAllocator {
    pub fn new() -> Self {
        Self {
            allocated_utf8: String::new(),
            allocated_wtf8: Wtf8Buf::new(),
            utf8_mapping: IndexVec::new(),
            wtf8_mapping: IndexVec::new(),
        }
    }

    pub fn alloc_utf8(&self) -> Utf8Builder<'_> {
        let start = self.allocated_utf8.len() as u32;
        Utf8Builder { start, alloc: self }
    }

    pub fn alloc_wtf8(&self) -> Wtf8Builder<'_> {
        let start = self.allocated_wtf8.len() as u32;
        Wtf8Builder { start, alloc: self }
    }

    pub fn get_utf8(&self, id: AtomId) -> &str {
        let id = self.utf8_mapping[id];
        &self.allocated_utf8[id.start as usize..id.end as usize]
    }

    pub fn get_wtf8(&self, id: Wtf8AtomId) -> &Wtf8 {
        let id = self.wtf8_mapping[id];
        &self
            .allocated_wtf8
            .slice(id.start as usize, id.end as usize)
    }
}

#[derive(Clone, Copy)]
struct AllocatedUtf8 {
    start: u32,
    end: u32,
}

pub struct Utf8Builder<'a> {
    start: u32,
    alloc: &'a mut StringAllocator,
}

impl<'a> Utf8Builder<'a> {
    pub fn finish(self) -> AtomId {
        let start = self.start;
        let end = self.alloc.allocated_utf8.len() as u32;
        let allocated = AllocatedUtf8 { start, end };
        self.alloc.utf8_mapping.push(allocated)
    }

    #[inline]
    pub fn push_str(&mut self, s: &str) {
        self.alloc.allocated_utf8.push_str(s);
    }

    #[inline]
    pub fn push(&mut self, c: char) {
        self.alloc.allocated_utf8.push(c);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.alloc.allocated_utf8.len() - self.start as usize
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Clone, Copy)]
struct AllocatedWtf8 {
    start: u32,
    end: u32,
}

pub struct Wtf8Builder<'a> {
    start: u32,
    alloc: &'a mut StringAllocator,
}

impl<'a> Wtf8Builder<'a> {
    pub fn finish(self) -> Wtf8AtomId {
        let start = self.start;
        let end = self.alloc.allocated_wtf8.len() as u32;
        let allocated = AllocatedWtf8 { start, end };
        self.alloc.wtf8_mapping.push(allocated)
    }

    #[inline]
    pub fn push(&mut self, c: CodePoint) {
        self.alloc.allocated_wtf8.push(c);
    }

    #[inline]
    pub fn push_str(&mut self, s: &str) {
        self.alloc.allocated_wtf8.push_str(s);
    }

    #[inline]
    pub fn push_char(&mut self, c: char) {
        self.alloc.allocated_wtf8.push_char(c);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.alloc.allocated_wtf8.len() - self.start as usize
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
