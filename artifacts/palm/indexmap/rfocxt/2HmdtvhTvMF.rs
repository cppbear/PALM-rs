pub use self::core::raw_entry_v1::{self, RawEntryApiV1};
pub use self::core::{Entry, IndexedEntry, OccupiedEntry, VacantEntry};
pub use self::iter::{
    Drain, IntoIter, IntoKeys, IntoValues, Iter, IterMut, IterMut2, Keys, Splice, Values,
    ValuesMut,
};
pub use self::mutable::MutableEntryKey;
pub use self::mutable::MutableKeys;
pub use self::slice::Slice;
#[cfg(feature = "rayon")]
pub use crate::rayon::map as rayon;
use ::core::cmp::Ordering;
use ::core::fmt;
use ::core::hash::{BuildHasher, Hash, Hasher};
use ::core::mem;
use ::core::ops::{Index, IndexMut, RangeBounds};
use alloc::boxed::Box;
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::collections::hash_map::RandomState;
use self::core::IndexMapCore;
use crate::util::{third, try_simplify_range};
use crate::{
    Bucket, Entries, Equivalent, GetDisjointMutError, HashValue, TryReserveError,
};
pub trait MutableKeys: private::Sealed {
    type Key;
    type Value;
    fn get_full_mut2<Q>(
        &mut self,
        key: &Q,
    ) -> Option<(usize, &mut Self::Key, &mut Self::Value)>
    where
        Q: ?Sized + Hash + Equivalent<Self::Key>;
    fn get_index_mut2(
        &mut self,
        index: usize,
    ) -> Option<(&mut Self::Key, &mut Self::Value)>;
    fn iter_mut2(&mut self) -> IterMut2<'_, Self::Key, Self::Value>;
    fn retain2<F>(&mut self, keep: F)
    where
        F: FnMut(&mut Self::Key, &mut Self::Value) -> bool;
}
pub trait RawEntryApiV1<K, V, S>: private::Sealed {
    fn raw_entry_v1(&self) -> RawEntryBuilder<'_, K, V, S>;
    fn raw_entry_mut_v1(&mut self) -> RawEntryBuilderMut<'_, K, V, S>;
}
trait Entries {
    type Entry;
    fn into_entries(self) -> Vec<Self::Entry>;
    fn as_entries(&self) -> &[Self::Entry];
    fn as_entries_mut(&mut self) -> &mut [Self::Entry];
    fn with_entries<F>(&mut self, f: F)
    where
        F: FnOnce(&mut [Self::Entry]);
}
#[cfg(not(feature = "std"))]
pub struct IndexMap<K, V, S> {
    pub(crate) core: IndexMapCore<K, V>,
    hash_builder: S,
}
pub struct Keys<'a, K, V> {
    iter: slice::Iter<'a, Bucket<K, V>>,
}
#[derive(Copy, Debug)]
struct Bucket<K, V> {
    hash: HashValue,
    key: K,
    value: V,
}
#[derive(Debug)]
pub(crate) struct IndexMapCore<K, V> {
    /// indices mapping from the entry hash to its index.
    indices: Indices,
    /// entries is a dense vec maintaining entry order.
    entries: Entries<K, V>,
}
impl<K, V, S> IndexMap<K, V, S> {
    #[inline]
    pub fn with_capacity_and_hasher(n: usize, hash_builder: S) -> Self {
        if n == 0 {
            Self::with_hasher(hash_builder)
        } else {
            IndexMap {
                core: IndexMapCore::with_capacity(n),
                hash_builder,
            }
        }
    }
    pub const fn with_hasher(hash_builder: S) -> Self {
        IndexMap {
            core: IndexMapCore::new(),
            hash_builder,
        }
    }
    pub fn capacity(&self) -> usize {}
    pub fn hasher(&self) -> &S {}
    #[inline]
    pub fn len(&self) -> usize {}
    #[inline]
    pub fn is_empty(&self) -> bool {}
    pub fn iter(&self) -> Iter<'_, K, V> {}
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {}
    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys::new(self.as_entries())
    }
    pub fn into_keys(self) -> IntoKeys<K, V> {}
    pub fn values(&self) -> Values<'_, K, V> {}
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {}
    pub fn into_values(self) -> IntoValues<K, V> {}
    pub fn clear(&mut self) {}
    pub fn truncate(&mut self, len: usize) {}
    #[track_caller]
    pub fn drain<R>(&mut self, range: R) -> Drain<'_, K, V>
    where
        R: RangeBounds<usize>,
    {}
    #[track_caller]
    pub fn split_off(&mut self, at: usize) -> Self
    where
        S: Clone,
    {
        Self {
            core: self.core.split_off(at),
            hash_builder: self.hash_builder.clone(),
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
impl<'a, K, V> Keys<'a, K, V> {
    pub(super) fn new(entries: &'a [Bucket<K, V>]) -> Self {
        Self { iter: entries.iter() }
    }
}
