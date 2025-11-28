use swc_common::BytePos;

oxc_index::define_index_type! {
    pub struct Wtf8AtomId = u32;
}

const STR_REF_ATOM_LO: BytePos = BytePos(0x80000000);

#[derive(Debug, Clone, Copy)]
pub struct Wtf8AtomRef {
    pub lo: BytePos,
    pub hi: BytePos,
}

impl Wtf8AtomRef {
    pub const fn new_ref(lo: BytePos, hi: BytePos) -> Self {
        Self { lo, hi }
    }

    pub const fn new_alloc(atom: Wtf8AtomId) -> Self {
        Self {
            lo: BytePos(atom.0),
            hi: STR_REF_ATOM_LO,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct OptionalWtf8AtomRef {
    pub lo: BytePos,
    pub hi: BytePos,
}

impl OptionalWtf8AtomRef {
    pub const fn new_ref(lo: BytePos, hi: BytePos) -> Self {
        Self { lo, hi }
    }

    pub const fn new_alloc(atom: Wtf8AtomId) -> Self {
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

    pub const fn to_option(self) -> Option<Wtf8AtomRef> {
        if self.hi.0 == u32::MAX {
            return None;
        }

        Some(Wtf8AtomRef {
            lo: self.lo,
            hi: self.hi,
        })
    }
}

impl From<Wtf8AtomRef> for OptionalWtf8AtomRef {
    fn from(value: Wtf8AtomRef) -> Self {
        Self::new_ref(value.lo, value.hi)
    }
}
