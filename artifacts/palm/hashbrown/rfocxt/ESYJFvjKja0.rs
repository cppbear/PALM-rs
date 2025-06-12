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
impl<T, A> HashTable<T, A>
where
    A: Allocator,
{
    pub const fn new_in(alloc: A) -> Self {
        Self {
            raw: RawTable::new_in(alloc),
        }
    }
    pub fn with_capacity_in(capacity: usize, alloc: A) -> Self {
        Self {
            raw: RawTable::with_capacity_in(capacity, alloc),
        }
    }
    pub fn allocator(&self) -> &A {}
    pub fn find(&self, hash: u64, eq: impl FnMut(&T) -> bool) -> Option<&T> {}
    pub fn find_mut(&mut self, hash: u64, eq: impl FnMut(&T) -> bool) -> Option<&mut T> {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn find_entry(
        &mut self,
        hash: u64,
        eq: impl FnMut(&T) -> bool,
    ) -> Result<OccupiedEntry<'_, T, A>, AbsentEntry<'_, T, A>> {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn entry(
        &mut self,
        hash: u64,
        eq: impl FnMut(&T) -> bool,
        hasher: impl Fn(&T) -> u64,
    ) -> Entry<'_, T, A> {}
    pub fn insert_unique(
        &mut self,
        hash: u64,
        value: T,
        hasher: impl Fn(&T) -> u64,
    ) -> OccupiedEntry<'_, T, A> {}
    pub fn clear(&mut self) {}
    pub fn shrink_to_fit(&mut self, hasher: impl Fn(&T) -> u64) {}
    pub fn shrink_to(&mut self, min_capacity: usize, hasher: impl Fn(&T) -> u64) {}
    pub fn reserve(&mut self, additional: usize, hasher: impl Fn(&T) -> u64) {
        self.raw.reserve(additional, hasher)
    }
    pub fn try_reserve(
        &mut self,
        additional: usize,
        hasher: impl Fn(&T) -> u64,
    ) -> Result<(), TryReserveError> {}
    pub fn capacity(&self) -> usize {}
    pub fn len(&self) -> usize {}
    pub fn is_empty(&self) -> bool {}
    pub fn iter(&self) -> Iter<'_, T> {}
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {}
    pub fn iter_hash(&self, hash: u64) -> IterHash<'_, T> {}
    pub fn iter_hash_mut(&mut self, hash: u64) -> IterHashMut<'_, T> {}
    pub fn retain(&mut self, mut f: impl FnMut(&mut T) -> bool) {}
    pub fn drain(&mut self) -> Drain<'_, T, A> {}
    pub fn extract_if<F>(&mut self, f: F) -> ExtractIf<'_, T, F, A>
    where
        F: FnMut(&mut T) -> bool,
    {}
    pub fn get_many_mut<const N: usize>(
        &mut self,
        hashes: [u64; N],
        eq: impl FnMut(usize, &T) -> bool,
    ) -> [Option<&'_ mut T>; N] {}
    pub unsafe fn get_many_unchecked_mut<const N: usize>(
        &mut self,
        hashes: [u64; N],
        eq: impl FnMut(usize, &T) -> bool,
    ) -> [Option<&'_ mut T>; N] {}
    #[inline]
    pub fn allocation_size(&self) -> usize {}
}
