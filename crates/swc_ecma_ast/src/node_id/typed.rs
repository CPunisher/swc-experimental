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

impl SubRange {
    pub(crate) unsafe fn cast_to_typed<T>(self) -> TypedSubRange<T> {
        TypedSubRange {
            inner: self,
            _phantom: PhantomData::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TypedNode<T> {
    inner: NodeId,
    _phantom: PhantomData<T>,
}

#[allow(dead_code)]
pub struct TypedOptionalNode<T> {
    inner: OptionalNodeId,
    _phantom: PhantomData<T>,
}

#[allow(dead_code)]
pub struct AnyExtraData<T> {
    inner: u64,
    _phantom: PhantomData<T>,
}
