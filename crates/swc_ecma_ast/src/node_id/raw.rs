use oxc_index::Idx;
use swc_common::BytePos;

macro_rules! define_optional_index_type {
    ($name:ident, $index_type:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(u32);

        impl Idx for $name {
            const MAX: usize = (u32::MAX - 1) as usize;

            unsafe fn from_usize_unchecked(idx: usize) -> Self {
                Self(idx as u32)
            }

            fn index(self) -> usize {
                self.0 as usize
            }
        }

        // impl $name {
        //     pub fn unwrap(self) -> Option<$index_type> {
        //         if self.0 == u32::MAX {
        //             return None;
        //         }
        //         Some($index_type(self.0))
        //     }
        // }
    };
}

oxc_index::define_index_type! {
    pub struct NodeId = u32;
}

define_optional_index_type!(OptionalNodeId, NodeId);

oxc_index::define_index_type! {
    pub struct AtomId = u32;
}

const STR_REF_ATOM_LO: BytePos = BytePos(0x80000000);

#[derive(Debug, Clone, Copy)]
pub struct AtomRef {
    pub lo: BytePos,
    pub hi: BytePos,
}

impl AtomRef {
    pub const fn new_ref(lo: BytePos, hi: BytePos) -> Self {
        Self { lo, hi }
    }

    pub const fn new_alloc(atom: AtomId) -> Self {
        Self {
            lo: STR_REF_ATOM_LO,
            hi: BytePos(atom.0),
        }
    }
}

oxc_index::define_index_type! {
    pub struct ExtraDataId = u32;
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct OptionalAtomRef {
    pub lo: BytePos,
    pub hi: BytePos,
}

impl OptionalAtomRef {
    pub const fn new_ref(lo: BytePos, hi: BytePos) -> Self {
        Self { lo, hi }
    }

    pub const fn new_alloc(atom: AtomId) -> Self {
        Self {
            lo: STR_REF_ATOM_LO,
            hi: BytePos(atom.0),
        }
    }

    pub const fn new_none() -> Self {
        Self {
            lo: BytePos(0),
            hi: BytePos(u32::MAX),
        }
    }

    pub const fn unwrap(self) -> Option<AtomRef> {
        if self.hi.0 == u32::MAX {
            return None;
        }

        Some(AtomRef {
            lo: self.lo,
            hi: self.hi,
        })
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct SubRange {
    pub start: ExtraDataId,
    pub end: ExtraDataId,
}

impl SubRange {
    pub const fn new(start: ExtraDataId, end: ExtraDataId) -> Self {
        Self { start, end }
    }
}

oxc_index::define_index_type! {
    pub struct BigIntId = u32;
}
