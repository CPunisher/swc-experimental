use swc_common::Span;

/// A reference to a utf8 string in the string allocator.
#[derive(Debug, Clone, Copy)]
pub struct Utf8Ref {
    /// The start pos of the utf8 string in the string allocator.
    lo: u32,
    /// The end pos of the utf8 string in the string allocator.
    hi: u32,
}

impl Utf8Ref {
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

/// An 8 bytes optimized version of `Option<Utf8Ref>` (12 bytes).
///
/// We regard it as `None` if `OptionalUtf8Ref::hi` is `u32::MAX`.
#[derive(Debug, Clone, Copy, Hash)]
pub struct OptionalUtf8Ref {
    lo: u32,
    hi: u32,
}

impl OptionalUtf8Ref {
    pub const fn new_ref(lo: u32, hi: u32) -> Self {
        Self { lo, hi }
    }

    pub const fn new_none() -> Self {
        Self {
            lo: 0,
            hi: u32::MAX,
        }
    }

    pub const fn to_option(self) -> Option<Utf8Ref> {
        if self.hi == u32::MAX {
            return None;
        }

        Some(Utf8Ref {
            lo: self.lo,
            hi: self.hi,
        })
    }
}

impl From<Utf8Ref> for OptionalUtf8Ref {
    fn from(value: Utf8Ref) -> Self {
        Self::new_ref(value.lo, value.hi)
    }
}
