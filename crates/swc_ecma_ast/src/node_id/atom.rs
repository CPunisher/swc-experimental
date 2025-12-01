use swc_common::Span;

#[derive(Debug, Clone, Copy)]
pub struct AtomRef {
    lo: u32,
    hi: u32,
}

impl AtomRef {
    pub const fn new_ref(lo: u32, hi: u32) -> Self {
        Self { lo, hi }
    }

    pub const fn new_from_span(span: Span) -> Self {
        Self::new_ref(span.lo.0, span.hi.0)
    }

    pub const fn new_empty() -> Self {
        Self { lo: 0, hi: 0 }
    }

    pub const fn lo(&self) -> u32 {
        self.lo
    }

    pub const fn hi(&self) -> u32 {
        self.hi
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct OptionalAtomRef {
    lo: u32,
    hi: u32,
}

impl OptionalAtomRef {
    pub const fn new_ref(lo: u32, hi: u32) -> Self {
        Self { lo, hi }
    }

    pub const fn new_none() -> Self {
        Self {
            lo: 0,
            hi: u32::MAX,
        }
    }

    pub const fn to_option(self) -> Option<AtomRef> {
        if self.hi == u32::MAX {
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
