use core::{fmt, iter::FusedIterator, marker::PhantomData};
use crate::{
    raw::{
        Allocator, Bucket, Global, InsertSlot, RawDrain, RawExtractIf, RawIntoIter,
        RawIter, RawIterHash, RawTable,
    },
    TryReserveError,
};
pub struct AbsentEntry<'a, T, A = Global>
where
    A: Allocator,
{
    table: &'a mut HashTable<T, A>,
}
pub struct HashTable<T, A = Global>
where
    A: Allocator,
{
    pub(crate) raw: RawTable<T, A>,
}
impl<'a, T, A> AbsentEntry<'a, T, A>
where
    A: Allocator,
{
    pub fn into_table(self) -> &'a mut HashTable<T, A> {
        self.table
    }
}
