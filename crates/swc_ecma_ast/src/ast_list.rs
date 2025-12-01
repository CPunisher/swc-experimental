use std::ops::{Index, IndexMut};

use oxc_index::IndexVec;

use crate::{AstNode, NodeId, NodeKind, OptionalNodeId};

pub(crate) struct NodeList {
    inner: IndexVec<NodeId, AstNode>,
    free_head: OptionalNodeId,
    num_elems: u32,
}

impl Default for NodeList {
    fn default() -> Self {
        Self {
            inner: Default::default(),
            free_head: OptionalNodeId::none(),
            num_elems: Default::default(),
        }
    }
}

impl NodeList {
    #[inline]
    pub fn add_node(&mut self, node: AstNode) -> NodeId {
        self.num_elems += 1;
        match self.free_head.to_option() {
            Some(node_id) => {
                debug_assert!(self.inner[node_id].kind == NodeKind::__FREED);
                self.free_head = unsafe { self.inner[node_id].data.next_free };
                self.inner[node_id] = node;
                node_id
            }
            None => self.inner.push(node),
        }
    }

    #[inline]
    pub fn free_node(&mut self, node_id: NodeId) {
        debug_assert!(self.inner[node_id].kind != NodeKind::__FREED);

        self.num_elems -= 1;
        self.inner[node_id].kind = NodeKind::__FREED;
        self.inner[node_id].data.next_free = self.free_head;
        self.free_head = OptionalNodeId::from(node_id);
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (NodeId, &AstNode)> {
        self.inner.iter_enumerated()
    }

    #[inline]
    pub fn len(&self) -> u32 {
        self.num_elems
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.inner.len()
    }
}

impl NodeList {
    pub(crate) unsafe fn get_unchecked(&self, node_id: NodeId) -> &AstNode {
        debug_assert!(node_id.index() < self.inner.len());
        unsafe { self.inner.as_raw_slice().get_unchecked(node_id.index()) }
    }

    pub(crate) unsafe fn get_unchecked_mut(&mut self, node_id: NodeId) -> &mut AstNode {
        unsafe {
            self.inner
                .as_raw_slice_mut()
                .get_unchecked_mut(node_id.index())
        }
    }
}

impl Index<NodeId> for NodeList {
    type Output = AstNode;

    fn index(&self, k: NodeId) -> &AstNode {
        &self.inner[k]
    }
}

impl IndexMut<NodeId> for NodeList {
    fn index_mut(&mut self, k: NodeId) -> &mut AstNode {
        &mut self.inner[k]
    }
}
