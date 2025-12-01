use std::{marker::PhantomData, ops::Deref};

use crate::{
    Ast, AstNode,
    node_id::{ExtraDataId, FromNodeId},
};

pub const EMPTY_SUB_RANGE: SubRange = SubRange {
    start: ExtraDataId::from_raw(0),
    end: ExtraDataId::from_raw(0),
};

#[derive(Debug, Clone, Copy, Hash)]
pub struct SubRange {
    pub start: ExtraDataId,
    pub end: ExtraDataId,
}

impl SubRange {
    pub(crate) unsafe fn cast_to_typed<T>(self) -> TypedSubRange<T> {
        TypedSubRange {
            inner: self,
            _phantom: PhantomData::default(),
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TypedSubRange<T> {
    pub(crate) inner: SubRange,
    pub(crate) _phantom: PhantomData<T>,
}

impl<T> TypedSubRange<T> {
    pub fn empty() -> Self {
        Self {
            inner: EMPTY_SUB_RANGE,
            _phantom: PhantomData::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.end.index() - self.start.index()
    }

    pub fn remove_first(&mut self) -> NodeExtraDataId<T> {
        assert!(self.len() >= 1);
        let start = self.start;
        self.inner.start = self.inner.start + 1;
        NodeExtraDataId {
            inner: start,
            _phantom: PhantomData::default(),
        }
    }

    pub fn get(&self, index: usize) -> NodeExtraDataId<T> {
        assert!(index < self.len());
        let id = self.start + index;
        NodeExtraDataId {
            inner: id,
            _phantom: PhantomData::default(),
        }
    }

    pub fn first(&self) -> Option<NodeExtraDataId<T>> {
        if self.is_empty() {
            return None;
        }

        Some(NodeExtraDataId {
            inner: self.start,
            _phantom: PhantomData::default(),
        })
    }

    pub fn last(&self) -> Option<NodeExtraDataId<T>> {
        if self.is_empty() {
            return None;
        }

        Some(NodeExtraDataId {
            inner: self.end - 1,
            _phantom: PhantomData::default(),
        })
    }

    pub fn iter<'a>(&self) -> TypedSubRangeIterator<T> {
        TypedSubRangeIterator {
            cur: self.start,
            end: self.end,
            _phantom: PhantomData::default(),
        }
    }

    pub fn split_off(&mut self, at: usize) -> Self {
        if at > self.len() {
            panic!(
                "Split point {} is larger than the length {}",
                at,
                self.len()
            );
        }

        let at = self.start + at;
        let original_end = self.inner.end;

        self.inner.end = at;
        TypedSubRange {
            inner: SubRange {
                start: at,
                end: original_end,
            },
            _phantom: PhantomData::default(),
        }
    }
}

impl<T> From<TypedSubRange<T>> for SubRange {
    fn from(value: TypedSubRange<T>) -> Self {
        value.inner
    }
}
impl<T> Deref for TypedSubRange<T> {
    type Target = SubRange;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct TypedSubRangeIterator<T> {
    cur: ExtraDataId,
    end: ExtraDataId,
    _phantom: PhantomData<T>,
}

impl<T: FromNodeId> Iterator for TypedSubRangeIterator<T> {
    type Item = NodeExtraDataId<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == self.end {
            return None;
        }

        let next = self.cur;
        debug_assert!(next < self.end);

        self.cur += 1;
        debug_assert!(self.cur <= self.end);

        Some(NodeExtraDataId {
            inner: next,
            _phantom: PhantomData::default(),
        })
    }
}

impl<T: FromNodeId> DoubleEndedIterator for TypedSubRangeIterator<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.cur == self.end {
            return None;
        }

        self.end -= 1;
        debug_assert!(self.cur <= self.end);

        let next = self.end;
        debug_assert!(next >= self.cur);

        Some(NodeExtraDataId {
            inner: next,
            _phantom: PhantomData::default(),
        })
    }
}

impl<T: FromNodeId> Iterator for TypedSubRangeIterator<Option<T>> {
    type Item = NodeExtraDataId<Option<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == self.end {
            return None;
        }

        let next = self.cur;
        debug_assert!(next < self.end);

        self.cur += 1;
        debug_assert!(self.cur <= self.end);

        Some(NodeExtraDataId {
            inner: next,
            _phantom: PhantomData::default(),
        })
    }
}

impl<T: FromNodeId> DoubleEndedIterator for TypedSubRangeIterator<Option<T>> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.cur == self.end {
            return None;
        }

        self.end -= 1;
        debug_assert!(self.cur <= self.end);

        let next = self.end;
        debug_assert!(next >= self.cur);

        Some(NodeExtraDataId {
            inner: next,
            _phantom: PhantomData::default(),
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NodeExtraDataId<T> {
    inner: ExtraDataId,
    _phantom: PhantomData<T>,
}

impl Ast {
    pub fn get_node_in_sub_range<T: FromNodeId>(&self, id: NodeExtraDataId<T>) -> T {
        let node_id = unsafe { self.extra_data[id.inner].node };
        unsafe { T::from_node_id_unchecked(node_id, self) }
    }

    pub fn get_opt_node_in_sub_range<T: FromNodeId>(
        &self,
        id: NodeExtraDataId<Option<T>>,
    ) -> Option<T> {
        let opt_node_id = unsafe { self.extra_data[id.inner].optional_node };
        opt_node_id.map(|node_id| unsafe { T::from_node_id_unchecked(node_id, self) })
    }

    pub fn get_raw_node_in_sub_range<T>(&self, id: NodeExtraDataId<T>) -> &AstNode {
        let node_id = unsafe { self.extra_data[id.inner].node };
        &self.nodes[node_id]
    }
}
