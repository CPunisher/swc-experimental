use std::ops::{Deref, DerefMut};

use oxc_index::IndexVec;
use swc_atoms::wtf8::{Wtf8, Wtf8Buf};
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

    pub fn alloc_utf8(&mut self) -> Utf8Builder<'_> {
        Utf8Builder {
            start: self.allocated_utf8.len() as u32,
            slice: &mut self.allocated_utf8,
        }
    }

    pub fn alloc_wtf8(&mut self) -> Wtf8Builder<'_> {
        Wtf8Builder {
            start: self.allocated_wtf8.len() as u32,
            slice: &mut self.allocated_wtf8,
        }
    }

    pub fn get_utf8(&self, id: AtomId) -> &str {
        let id = self.utf8_mapping[id];
        &self.allocated_utf8[id.start as usize..(id.start + id.len) as usize]
    }

    pub fn get_wtf8(&self, id: Wtf8AtomId) -> &Wtf8 {
        let id = self.wtf8_mapping[id];
        &self
            .allocated_wtf8
            .slice(id.start as usize, (id.start + id.len) as usize)
    }
}

#[derive(Clone, Copy)]
pub struct AllocatedUtf8 {
    start: u32,
    len: u32,
}

pub struct Utf8Builder<'a> {
    start: u32,
    slice: &'a mut str,
}

impl<'a> Utf8Builder<'a> {
    pub fn finish(self) -> AllocatedUtf8 {
        let start = self.start;
        let len = self.slice.len() as u32;
        AllocatedUtf8 { start, len }
    }
}

impl Deref for Utf8Builder<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.slice
    }
}

impl DerefMut for Utf8Builder<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.slice
    }
}

#[derive(Clone, Copy)]
pub struct AllocatedWtf8 {
    start: u32,
    len: u32,
}

pub struct Wtf8Builder<'a> {
    start: u32,
    slice: &'a mut Wtf8Buf,
}

impl<'a> Wtf8Builder<'a> {
    pub fn finish(self) -> AllocatedWtf8 {
        let start = self.start;
        let len = self.slice.len() as u32;
        AllocatedWtf8 { start, len }
    }
}

impl Deref for Wtf8Builder<'_> {
    type Target = Wtf8Buf;

    fn deref(&self) -> &Self::Target {
        self.slice
    }
}

impl DerefMut for Wtf8Builder<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.slice
    }
}
