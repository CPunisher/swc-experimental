use std::marker::PhantomData;

use crate::node_id::{NodeId, OptionalNodeId, SubRange};

pub struct TypedSubRange<T> {
    inner: SubRange,
    _phantom: PhantomData<T>,
}

impl<T> From<TypedSubRange<T>> for SubRange {
    fn from(value: TypedSubRange<T>) -> Self {
        value.inner
    }
}

pub struct TypedNodeId<T> {
    inner: NodeId,
    _phantom: PhantomData<T>,
}

impl<T> From<TypedNodeId<T>> for NodeId {
    fn from(value: TypedNodeId<T>) -> Self {
        value.inner
    }
}

pub struct TypedOptionalNodeId<T> {
    inner: OptionalNodeId,
    _phantom: PhantomData<T>,
}

impl<T> From<TypedOptionalNodeId<T>> for OptionalNodeId {
    fn from(value: TypedOptionalNodeId<T>) -> Self {
        value.inner
    }
}

impl NodeId {
    pub(crate) unsafe fn cast_to_typed<T>(self) -> TypedNodeId<T> {
        TypedNodeId {
            inner: self,
            _phantom: PhantomData::default(),
        }
    }
}
