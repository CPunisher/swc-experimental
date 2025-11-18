use swc_common::BytePos;

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
            lo: BytePos(atom.0),
            hi: STR_REF_ATOM_LO,
        }
    }
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

    pub const fn to_option(self) -> Option<AtomRef> {
        if self.hi.0 == u32::MAX {
            return None;
        }

        Some(AtomRef {
            lo: self.lo,
            hi: self.hi,
        })
    }
}

impl From<AtomRef> for OptionalAtomRef {
    fn from(value: AtomRef) -> Self {
        Self::new_ref(value.lo, value.hi)
    }
}
