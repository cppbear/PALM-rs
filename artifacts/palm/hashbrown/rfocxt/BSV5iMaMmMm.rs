use core::{fmt, iter::FusedIterator, marker::PhantomData};
use crate::{
    raw::{
        Allocator, Bucket, Global, InsertSlot, RawDrain, RawExtractIf, RawIntoIter,
        RawIter, RawIterHash, RawTable,
    },
    TryReserveError,
};
#[allow(clippy::missing_safety_doc)]
pub unsafe trait Allocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()>;
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout);
}
pub struct OccupiedEntry<'a, T, A = Global>
where
    A: Allocator,
{
    hash: u64,
    bucket: Bucket<T>,
    table: &'a mut HashTable<T, A>,
}
pub struct Bucket<T> {
    ptr: NonNull<T>,
}
pub struct HashTable<T, A = Global>
where
    A: Allocator,
{
    pub(crate) raw: RawTable<T, A>,
}
impl<T: fmt::Debug, A: Allocator> fmt::Debug for OccupiedEntry<'_, T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OccupiedEntry").field("value", self.get()).finish()
    }
}
impl<'a, T, A> OccupiedEntry<'a, T, A>
where
    A: Allocator,
{
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn remove(self) -> (T, VacantEntry<'a, T, A>) {}
    #[inline]
    pub fn get(&self) -> &T {
        unsafe { self.bucket.as_ref() }
    }
    #[inline]
    pub fn get_mut(&mut self) -> &mut T {}
    pub fn into_mut(self) -> &'a mut T {}
    pub fn into_table(self) -> &'a mut HashTable<T, A> {}
}
