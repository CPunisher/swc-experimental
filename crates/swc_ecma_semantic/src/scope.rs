use bitflags::bitflags;
use swc_experimental_ecma_ast::NodeId;

oxc_index::define_index_type! {
    pub struct ScopeId = u32;
}

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
