#[derive(Debug, Clone, Copy)]
pub struct Wtf8AtomRef {
    lo: u32,
    hi: u32,
}

impl Wtf8AtomRef {
    pub const fn new_ref(lo: u32, hi: u32) -> Self {
        Self { lo, hi }
    }

    pub const fn lo(&self) -> u32 {
        self.lo
    }

    pub const fn hi(&self) -> u32 {
        self.hi
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct OptionalWtf8AtomRef {
    lo: u32,
    hi: u32,
}
impl OptionalWtf8AtomRef {
    pub const fn new_ref(lo: u32, hi: u32) -> Self {
        Self { lo, hi }
    }

    pub const fn new_none() -> Self {
        Self {
            lo: 0,
            hi: u32::MAX,
        }
    }

    pub const fn to_option(self) -> Option<Wtf8AtomRef> {
        if self.hi == u32::MAX {
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
