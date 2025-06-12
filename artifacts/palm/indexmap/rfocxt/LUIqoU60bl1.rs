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
#[repr(transparent)]
pub struct Slice<K, V> {
    pub(crate) entries: [Bucket<K, V>],
}
#[derive(Debug)]
pub(crate) struct IndexMapCore<K, V> {
    /// indices mapping from the entry hash to its index.
    indices: Indices,
    /// entries is a dense vec maintaining entry order.
    entries: Entries<K, V>,
}
impl<K, V, S> IndexMap<K, V, S> {
    pub fn pop(&mut self) -> Option<(K, V)> {}
    pub fn retain<F>(&mut self, mut keep: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {}
    pub fn sort_keys(&mut self)
    where
        K: Ord,
    {}
    pub fn sort_by<F>(&mut self, mut cmp: F)
    where
        F: FnMut(&K, &V, &K, &V) -> Ordering,
    {}
    pub fn sorted_by<F>(self, mut cmp: F) -> IntoIter<K, V>
    where
        F: FnMut(&K, &V, &K, &V) -> Ordering,
    {}
    pub fn sort_unstable_keys(&mut self)
    where
        K: Ord,
    {}
    pub fn sort_unstable_by<F>(&mut self, mut cmp: F)
    where
        F: FnMut(&K, &V, &K, &V) -> Ordering,
    {}
    #[inline]
    pub fn sorted_unstable_by<F>(self, mut cmp: F) -> IntoIter<K, V>
    where
        F: FnMut(&K, &V, &K, &V) -> Ordering,
    {}
    pub fn sort_by_cached_key<T, F>(&mut self, mut sort_key: F)
    where
        T: Ord,
        F: FnMut(&K, &V) -> T,
    {}
    pub fn binary_search_keys(&self, x: &K) -> Result<usize, usize>
    where
        K: Ord,
    {
        self.as_slice().binary_search_keys(x)
    }
    #[inline]
    pub fn binary_search_by<'a, F>(&'a self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a K, &'a V) -> Ordering,
    {}
    #[inline]
    pub fn binary_search_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a K, &'a V) -> B,
        B: Ord,
    {}
    #[must_use]
    pub fn partition_point<P>(&self, pred: P) -> usize
    where
        P: FnMut(&K, &V) -> bool,
    {}
    pub fn reverse(&mut self) {}
    pub fn as_slice(&self) -> &Slice<K, V> {
        Slice::from_slice(self.as_entries())
    }
    pub fn as_mut_slice(&mut self) -> &mut Slice<K, V> {}
    pub fn into_boxed_slice(self) -> Box<Slice<K, V>> {}
    pub fn get_index(&self, index: usize) -> Option<(&K, &V)> {}
    pub fn get_index_mut(&mut self, index: usize) -> Option<(&K, &mut V)> {}
    pub fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, K, V>> {}
    pub fn get_disjoint_indices_mut<const N: usize>(
        &mut self,
        indices: [usize; N],
    ) -> Result<[(&K, &mut V); N], GetDisjointMutError> {}
    pub fn get_range<R: RangeBounds<usize>>(&self, range: R) -> Option<&Slice<K, V>> {}
    pub fn get_range_mut<R: RangeBounds<usize>>(
        &mut self,
        range: R,
    ) -> Option<&mut Slice<K, V>> {}
    pub fn first(&self) -> Option<(&K, &V)> {}
    pub fn first_mut(&mut self) -> Option<(&K, &mut V)> {}
    pub fn first_entry(&mut self) -> Option<IndexedEntry<'_, K, V>> {}
    pub fn last(&self) -> Option<(&K, &V)> {}
    pub fn last_mut(&mut self) -> Option<(&K, &mut V)> {}
    pub fn last_entry(&mut self) -> Option<IndexedEntry<'_, K, V>> {}
    pub fn swap_remove_index(&mut self, index: usize) -> Option<(K, V)> {}
    pub fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {}
    #[track_caller]
    pub fn move_index(&mut self, from: usize, to: usize) {}
    #[track_caller]
    pub fn swap_indices(&mut self, a: usize, b: usize) {}
}
impl<K, V> Slice<K, V> {
    pub(crate) fn into_entries(self: Box<Self>) -> Vec<Bucket<K, V>> {}
    pub const fn new<'a>() -> &'a Self {
        Self::from_slice(&[])
    }
    pub fn new_mut<'a>() -> &'a mut Self {
        Self::from_mut_slice(&mut [])
    }
    #[inline]
    pub const fn len(&self) -> usize {}
    #[inline]
    pub const fn is_empty(&self) -> bool {}
    pub fn get_index(&self, index: usize) -> Option<(&K, &V)> {}
    pub fn get_index_mut(&mut self, index: usize) -> Option<(&K, &mut V)> {}
    pub fn get_range<R: RangeBounds<usize>>(&self, range: R) -> Option<&Self> {
        let range = try_simplify_range(range, self.entries.len())?;
        self.entries.get(range).map(Slice::from_slice)
    }
    pub fn get_range_mut<R: RangeBounds<usize>>(
        &mut self,
        range: R,
    ) -> Option<&mut Self> {
        let range = try_simplify_range(range, self.entries.len())?;
        self.entries.get_mut(range).map(Slice::from_mut_slice)
    }
    pub fn first(&self) -> Option<(&K, &V)> {}
    pub fn first_mut(&mut self) -> Option<(&K, &mut V)> {}
    pub fn last(&self) -> Option<(&K, &V)> {}
    pub fn last_mut(&mut self) -> Option<(&K, &mut V)> {}
    pub fn split_at(&self, index: usize) -> (&Self, &Self) {
        let (first, second) = self.entries.split_at(index);
        (Self::from_slice(first), Self::from_slice(second))
    }
    pub fn split_at_mut(&mut self, index: usize) -> (&mut Self, &mut Self) {
        let (first, second) = self.entries.split_at_mut(index);
        (Self::from_mut_slice(first), Self::from_mut_slice(second))
    }
    pub fn split_first(&self) -> Option<((&K, &V), &Self)> {
        if let [first, rest @ ..] = &self.entries {
            Some((first.refs(), Self::from_slice(rest)))
        } else {
            None
        }
    }
    pub fn split_first_mut(&mut self) -> Option<((&K, &mut V), &mut Self)> {
        if let [first, rest @ ..] = &mut self.entries {
            Some((first.ref_mut(), Self::from_mut_slice(rest)))
        } else {
            None
        }
    }
    pub fn split_last(&self) -> Option<((&K, &V), &Self)> {
        if let [rest @ .., last] = &self.entries {
            Some((last.refs(), Self::from_slice(rest)))
        } else {
            None
        }
    }
    pub fn split_last_mut(&mut self) -> Option<((&K, &mut V), &mut Self)> {
        if let [rest @ .., last] = &mut self.entries {
            Some((last.ref_mut(), Self::from_mut_slice(rest)))
        } else {
            None
        }
    }
    pub fn iter(&self) -> Iter<'_, K, V> {}
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {}
    pub fn keys(&self) -> Keys<'_, K, V> {}
    pub fn into_keys(self: Box<Self>) -> IntoKeys<K, V> {}
    pub fn values(&self) -> Values<'_, K, V> {}
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {}
    pub fn into_values(self: Box<Self>) -> IntoValues<K, V> {}
    pub fn binary_search_keys(&self, x: &K) -> Result<usize, usize>
    where
        K: Ord,
    {
        self.binary_search_by(|p, _| p.cmp(x))
    }
    #[inline]
    pub fn binary_search_by<'a, F>(&'a self, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a K, &'a V) -> Ordering,
    {}
    #[inline]
    pub fn binary_search_by_key<'a, B, F>(
        &'a self,
        b: &B,
        mut f: F,
    ) -> Result<usize, usize>
    where
        F: FnMut(&'a K, &'a V) -> B,
        B: Ord,
    {}
    #[must_use]
    pub fn partition_point<P>(&self, mut pred: P) -> usize
    where
        P: FnMut(&K, &V) -> bool,
    {}
    pub fn get_disjoint_mut<const N: usize>(
        &mut self,
        indices: [usize; N],
    ) -> Result<[(&K, &mut V); N], GetDisjointMutError> {}
    #[allow(unsafe_code)]
    pub(crate) fn get_disjoint_opt_mut<const N: usize>(
        &mut self,
        indices: [Option<usize>; N],
    ) -> Result<[Option<(&K, &mut V)>; N], GetDisjointMutError> {}
}
