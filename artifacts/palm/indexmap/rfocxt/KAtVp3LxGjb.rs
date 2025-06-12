use super::{Bucket, Entries, IndexSet, Slice};
use alloc::vec::{self, Vec};
use core::fmt;
use core::hash::{BuildHasher, Hash};
use core::iter::{Chain, FusedIterator};
use core::ops::RangeBounds;
use core::slice::Iter as SliceIter;
pub struct Union<'a, T, S> {
    iter: Chain<Iter<'a, T>, Difference<'a, T, S>>,
}
#[cfg(not(feature = "std"))]
pub struct IndexSet<T, S> {
    pub(crate) map: IndexMap<T, (), S>,
}
pub struct Difference<'a, T, S> {
    iter: Iter<'a, T>,
    other: &'a IndexSet<T, S>,
}
pub struct Iter<'a, T> {
    iter: SliceIter<'a, Bucket<T>>,
}
pub struct Iter<'a, K, V> {
    iter: slice::Iter<'a, Bucket<K, V>>,
}
impl<'a, T, S> Union<'a, T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    pub(super) fn new<S2>(set1: &'a IndexSet<T, S>, set2: &'a IndexSet<T, S2>) -> Self
    where
        S2: BuildHasher,
    {
        Self {
            iter: set1.iter().chain(set2.difference(set1)),
        }
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
