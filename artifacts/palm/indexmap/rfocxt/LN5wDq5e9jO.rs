use super::{Bucket, Entries, IndexSet, Slice};
use alloc::vec::{self, Vec};
use core::fmt;
use core::hash::{BuildHasher, Hash};
use core::iter::{Chain, FusedIterator};
use core::ops::RangeBounds;
use core::slice::Iter as SliceIter;
pub struct Intersection<'a, T, S> {
    iter: Iter<'a, T>,
    other: &'a IndexSet<T, S>,
}
#[cfg(not(feature = "std"))]
pub struct IndexSet<T, S> {
    pub(crate) map: IndexMap<T, (), S>,
}
pub struct Iter<'a, T> {
    iter: SliceIter<'a, Bucket<T>>,
}
pub struct Iter<'a, K, V> {
    iter: slice::Iter<'a, Bucket<K, V>>,
}
impl<'a, T, S> Intersection<'a, T, S> {
    pub(super) fn new<S1>(set: &'a IndexSet<T, S1>, other: &'a IndexSet<T, S>) -> Self {
        Self { iter: set.iter(), other }
    }
}
impl<T, S> IndexSet<T, S> {
    pub fn with_capacity_and_hasher(n: usize, hash_builder: S) -> Self {
        IndexSet {
            map: IndexMap::with_capacity_and_hasher(n, hash_builder),
        }
    }
    pub const fn with_hasher(hash_builder: S) -> Self {
        IndexSet {
            map: IndexMap::with_hasher(hash_builder),
        }
    }
    pub fn capacity(&self) -> usize {}
    pub fn hasher(&self) -> &S {}
    pub fn len(&self) -> usize {}
    pub fn is_empty(&self) -> bool {}
    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(self.as_entries())
    }
    pub fn clear(&mut self) {}
    pub fn truncate(&mut self, len: usize) {}
    #[track_caller]
    pub fn drain<R>(&mut self, range: R) -> Drain<'_, T>
    where
        R: RangeBounds<usize>,
    {}
    #[track_caller]
    pub fn split_off(&mut self, at: usize) -> Self
    where
        S: Clone,
    {
        Self {
            map: self.map.split_off(at),
        }
    }
    pub fn reserve(&mut self, additional: usize) {}
    pub fn reserve_exact(&mut self, additional: usize) {}
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {}
    pub fn try_reserve_exact(
        &mut self,
        additional: usize,
    ) -> Result<(), TryReserveError> {}
    pub fn shrink_to_fit(&mut self) {}
    pub fn shrink_to(&mut self, min_capacity: usize) {}
}
