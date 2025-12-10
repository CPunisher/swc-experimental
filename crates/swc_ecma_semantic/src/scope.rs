use std::num::NonZeroU32;

use bitflags::bitflags;
use oxc_index::Idx;
use swc_core::common::SyntaxContext;
use swc_experimental_ecma_ast::NodeId;

pub struct Scope {
    parent: Option<ScopeId>,
    flags: ScopeFlags,
    decaration: NodeId,
}

impl Scope {
    pub(crate) fn new(parent: Option<ScopeId>, flags: ScopeFlags, decaration: NodeId) -> Self {
        Self {
            parent: None,
            flags,
            decaration,
        }
    }

    #[inline]
    pub fn parent(&self) -> Option<ScopeId> {
        self.parent
    }

    #[inline]
    pub fn flags(&self) -> ScopeFlags {
        self.flags
    }
}

bitflags! {
   #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct ScopeFlags: u8 {
        const StrictMode = 1 << 0;
        const Block      = 1 << 1;
        const Fn         = 1 << 2;
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScopeId(pub(crate) NonZeroU32);

impl Idx for ScopeId {
    const MAX: usize = u32::MAX as usize;

    unsafe fn from_usize_unchecked(idx: usize) -> Self {
        unsafe { Self(NonZeroU32::new_unchecked(idx as u32 + 1)) }
    }

    fn index(self) -> usize {
        self.0.get() as usize - 1
    }
}

impl ScopeId {
    pub fn to_ctxt(self) -> SyntaxContext {
        SyntaxContext::from_u32(self.0.get())
    }
}
