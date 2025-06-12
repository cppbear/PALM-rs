use core::{fmt, iter::FusedIterator, marker::PhantomData};
use crate::{
    raw::{
        Allocator, Bucket, Global, InsertSlot, RawDrain, RawExtractIf, RawIntoIter,
        RawIter, RawIterHash, RawTable,
    },
    TryReserveError,
};
pub struct VacantEntry<'a, T, A = Global>
where
    A: Allocator,
{
    hash: u64,
    insert_slot: InsertSlot,
    table: &'a mut HashTable<T, A>,
}
pub struct HashTable<T, A = Global>
where
    A: Allocator,
{
    pub(crate) raw: RawTable<T, A>,
}
pub struct InsertSlot {
    index: usize,
}
impl<'a, T, A> VacantEntry<'a, T, A>
where
    A: Allocator,
{
    #[inline]
    pub fn insert(self, value: T) -> OccupiedEntry<'a, T, A> {}
    pub fn into_table(self) -> &'a mut HashTable<T, A> {
        self.table
    }
}
