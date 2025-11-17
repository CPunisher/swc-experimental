use crate::scope::ScopeId;

pub struct NodeInfo {
    scope_id: ScopeId,
}

impl NodeInfo {
    pub(crate) fn new(scope_id: ScopeId) -> Self {
        Self { scope_id }
    }

    #[inline]
    pub fn scope_id(&self) -> ScopeId {
        self.scope_id
    }
}
