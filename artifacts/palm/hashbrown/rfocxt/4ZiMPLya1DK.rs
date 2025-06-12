use core::{fmt, iter::FusedIterator, marker::PhantomData};
use crate::{
    raw::{
        Allocator, Bucket, Global, InsertSlot, RawDrain, RawExtractIf, RawIntoIter,
        RawIter, RawIterHash, RawTable,
    },
    TryReserveError,
};
pub struct HashTable<T, A = Global>
where
    A: Allocator,
{
    pub(crate) raw: RawTable<T, A>,
}
pub struct RawTable<T, A: Allocator = Global> {
    table: RawTableInner,
    alloc: A,
    marker: PhantomData<T>,
}
impl<T> HashTable<T, Global> {
    pub const fn new() -> Self {
        Self { raw: RawTable::new() }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            raw: RawTable::with_capacity(capacity),
        }
    }
}
impl<T> RawTable<T, Global> {
    #[inline]
    pub const fn new() -> Self {
        Self {
            table: RawTableInner::NEW,
            alloc: Global,
            marker: PhantomData,
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_in(capacity, Global)
    }
}
