use crate::Ast;

pub trait GetNodeId {
    fn node_id(&self) -> NodeId;
}

pub trait GetOptionalNodeId {
    fn optional_node_id(&self) -> OptionalNodeId;
}

impl<T: GetNodeId> GetOptionalNodeId for T {
    fn optional_node_id(&self) -> OptionalNodeId {
        self.node_id().into()
    }
}

pub trait FromNodeId {
    fn from_node_id(id: NodeId, ast: &Ast) -> Self;

    /// # Safety
    /// 1. The caller should ensure that `node.kind` is corresponding to `Self`
    unsafe fn from_node_id_unchecked(id: NodeId, ast: &Ast) -> Self;
}

oxc_index::define_index_type! {
    pub struct NodeId = u32;
}

/// An 4 bytes optimized version of `Option<NodeId>` (8 bytes)
///
/// We regard it as `None` if the inner is `u32::MAX`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OptionalNodeId(u32);

impl From<NodeId> for OptionalNodeId {
    fn from(value: NodeId) -> Self {
        Self(value.0)
    }
}

impl OptionalNodeId {
    #[inline]
    pub fn none() -> Self {
        Self(u32::MAX)
    }

    #[inline]
    pub fn to_option(self) -> Option<NodeId> {
        if self.0 == u32::MAX {
            return None;
        }

        Some(NodeId(self.0))
    }

    #[inline]
    pub fn map<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(NodeId) -> U,
    {
        if self.0 == u32::MAX {
            return None;
        }

        Some(f(NodeId(self.0)))
    }
}
