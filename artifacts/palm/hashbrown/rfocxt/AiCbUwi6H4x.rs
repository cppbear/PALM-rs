use core::{fmt, iter::FusedIterator, marker::PhantomData};
use crate::{
    raw::{
        Allocator, Bucket, Global, InsertSlot, RawDrain, RawExtractIf, RawIntoIter,
        RawIter, RawIterHash, RawTable,
    },
    TryReserveError,
};
pub struct OccupiedEntry<'a, T, A = Global>
where
    A: Allocator,
{
    hash: u64,
    bucket: Bucket<T>,
    table: &'a mut HashTable<T, A>,
}
pub struct HashTable<T, A = Global>
where
    A: Allocator,
{
    pub(crate) raw: RawTable<T, A>,
}
pub struct Bucket<T> {
    ptr: NonNull<T>,
}
impl<'a, T, A> OccupiedEntry<'a, T, A>
where
    A: Allocator,
{
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn remove(self) -> (T, VacantEntry<'a, T, A>) {}
    #[inline]
    pub fn get(&self) -> &T {}
    #[inline]
    pub fn get_mut(&mut self) -> &mut T {}
    pub fn into_mut(self) -> &'a mut T {}
    pub fn into_table(self) -> &'a mut HashTable<T, A> {
        self.table
    }
}
