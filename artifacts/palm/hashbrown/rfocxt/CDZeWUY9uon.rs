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
    pub fn reserve(&mut self, additional: usize, hasher: impl Fn(&T) -> u64) {}
    pub fn try_reserve(
        &mut self,
        additional: usize,
        hasher: impl Fn(&T) -> u64,
    ) -> Result<(), TryReserveError> {}
    pub fn capacity(&self) -> usize {}
    pub fn len(&self) -> usize {}
    pub fn is_empty(&self) -> bool {
        self.raw.is_empty()
    }
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
impl<T, A: Allocator> RawTable<T, A> {
    const TABLE_LAYOUT: TableLayout = TableLayout::new::<T>();
    #[inline]
    pub const fn new_in(alloc: A) -> Self {
        Self {
            table: RawTableInner::NEW,
            alloc,
            marker: PhantomData,
        }
    }
    #[cfg_attr(feature = "inline-more", inline)]
    unsafe fn new_uninitialized(
        alloc: A,
        buckets: usize,
        fallibility: Fallibility,
    ) -> Result<Self, TryReserveError> {
        debug_assert!(buckets.is_power_of_two());
        Ok(Self {
            table: RawTableInner::new_uninitialized(
                &alloc,
                Self::TABLE_LAYOUT,
                buckets,
                fallibility,
            )?,
            alloc,
            marker: PhantomData,
        })
    }
    pub fn with_capacity_in(capacity: usize, alloc: A) -> Self {
        Self {
            table: RawTableInner::with_capacity(&alloc, Self::TABLE_LAYOUT, capacity),
            alloc,
            marker: PhantomData,
        }
    }
    #[inline]
    pub fn allocator(&self) -> &A {}
    #[inline]
    pub fn data_end(&self) -> NonNull<T> {}
    #[inline]
    #[cfg(feature = "nightly")]
    pub unsafe fn data_start(&self) -> NonNull<T> {}
    #[inline]
    pub fn allocation_size(&self) -> usize {}
    #[inline]
    pub unsafe fn bucket_index(&self, bucket: &Bucket<T>) -> usize {}
    #[inline]
    pub unsafe fn bucket(&self, index: usize) -> Bucket<T> {}
    #[cfg_attr(feature = "inline-more", inline)]
    unsafe fn erase_no_drop(&mut self, item: &Bucket<T>) {}
    #[cfg_attr(feature = "inline-more", inline)]
    #[allow(clippy::needless_pass_by_value)]
    pub unsafe fn erase(&mut self, item: Bucket<T>) {}
    #[cfg_attr(feature = "inline-more", inline)]
    #[allow(clippy::needless_pass_by_value)]
    pub unsafe fn remove(&mut self, item: Bucket<T>) -> (T, InsertSlot) {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn remove_entry(&mut self, hash: u64, eq: impl FnMut(&T) -> bool) -> Option<T> {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn clear_no_drop(&mut self) {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn clear(&mut self) {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn shrink_to(&mut self, min_size: usize, hasher: impl Fn(&T) -> u64) {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn reserve(&mut self, additional: usize, hasher: impl Fn(&T) -> u64) {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn try_reserve(
        &mut self,
        additional: usize,
        hasher: impl Fn(&T) -> u64,
    ) -> Result<(), TryReserveError> {}
    #[cold]
    #[inline(never)]
    unsafe fn reserve_rehash(
        &mut self,
        additional: usize,
        hasher: impl Fn(&T) -> u64,
        fallibility: Fallibility,
    ) -> Result<(), TryReserveError> {}
    unsafe fn resize(
        &mut self,
        capacity: usize,
        hasher: impl Fn(&T) -> u64,
        fallibility: Fallibility,
    ) -> Result<(), TryReserveError> {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn insert(
        &mut self,
        hash: u64,
        value: T,
        hasher: impl Fn(&T) -> u64,
    ) -> Bucket<T> {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn insert_entry(
        &mut self,
        hash: u64,
        value: T,
        hasher: impl Fn(&T) -> u64,
    ) -> &mut T {}
    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "rustc-internal-api")]
    pub unsafe fn insert_no_grow(&mut self, hash: u64, value: T) -> Bucket<T> {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub unsafe fn replace_bucket_with<F>(&mut self, bucket: Bucket<T>, f: F) -> bool
    where
        F: FnOnce(T) -> Option<T>,
    {}
    #[inline]
    pub fn find_or_find_insert_slot(
        &mut self,
        hash: u64,
        mut eq: impl FnMut(&T) -> bool,
        hasher: impl Fn(&T) -> u64,
    ) -> Result<Bucket<T>, InsertSlot> {}
    #[inline]
    pub unsafe fn insert_in_slot(
        &mut self,
        hash: u64,
        slot: InsertSlot,
        value: T,
    ) -> Bucket<T> {}
    #[inline]
    pub fn find(&self, hash: u64, mut eq: impl FnMut(&T) -> bool) -> Option<Bucket<T>> {}
    #[inline]
    pub fn get(&self, hash: u64, eq: impl FnMut(&T) -> bool) -> Option<&T> {}
    #[inline]
    pub fn get_mut(&mut self, hash: u64, eq: impl FnMut(&T) -> bool) -> Option<&mut T> {}
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
    unsafe fn get_many_mut_pointers<const N: usize>(
        &mut self,
        hashes: [u64; N],
        mut eq: impl FnMut(usize, &T) -> bool,
    ) -> [Option<NonNull<T>>; N] {}
    #[inline]
    pub fn capacity(&self) -> usize {}
    #[inline]
    pub fn len(&self) -> usize {}
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    #[inline]
    pub fn buckets(&self) -> usize {}
    #[inline]
    pub unsafe fn is_bucket_full(&self, index: usize) -> bool {}
    #[inline]
    pub unsafe fn iter(&self) -> RawIter<T> {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub unsafe fn iter_hash(&self, hash: u64) -> RawIterHash<T> {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn drain(&mut self) -> RawDrain<'_, T, A> {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub unsafe fn drain_iter_from(&mut self, iter: RawIter<T>) -> RawDrain<'_, T, A> {}
    pub unsafe fn into_iter_from(self, iter: RawIter<T>) -> RawIntoIter<T, A> {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub(crate) fn into_allocation(self) -> Option<(NonNull<u8>, Layout, A)> {}
}
