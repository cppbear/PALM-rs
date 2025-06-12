use super::{Entries, RefMut};
use crate::{Equivalent, HashValue, IndexMap};
use core::fmt;
use core::hash::{BuildHasher, Hash, Hasher};
use core::marker::PhantomData;
use core::mem;
use hashbrown::hash_table;
pub struct RawEntryBuilder<'a, K, V, S> {
    map: &'a IndexMap<K, V, S>,
}
#[cfg(not(feature = "std"))]
pub struct IndexMap<K, V, S> {
    pub(crate) core: IndexMapCore<K, V>,
    hash_builder: S,
}
impl<'a, K, V, S> RawEntryBuilder<'a, K, V, S> {
    pub fn from_key<Q>(self, key: &Q) -> Option<(&'a K, &'a V)>
    where
        S: BuildHasher,
        Q: ?Sized + Hash + Equivalent<K>,
    {}
    pub fn from_key_hashed_nocheck<Q>(self, hash: u64, key: &Q) -> Option<(&'a K, &'a V)>
    where
        Q: ?Sized + Equivalent<K>,
    {}
    pub fn from_hash<F>(self, hash: u64, is_match: F) -> Option<(&'a K, &'a V)>
    where
        F: FnMut(&K) -> bool,
    {}
    pub fn from_hash_full<F>(
        self,
        hash: u64,
        is_match: F,
    ) -> Option<(usize, &'a K, &'a V)>
    where
        F: FnMut(&K) -> bool,
    {
        let map = self.map;
        let i = self.index_from_hash(hash, is_match)?;
        let (key, value) = map.get_index(i)?;
        Some((i, key, value))
    }
    pub fn index_from_hash<F>(self, hash: u64, mut is_match: F) -> Option<usize>
    where
        F: FnMut(&K) -> bool,
    {}
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
    {}
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
    pub fn as_slice(&self) -> &Slice<K, V> {}
    pub fn as_mut_slice(&mut self) -> &mut Slice<K, V> {}
    pub fn into_boxed_slice(self) -> Box<Slice<K, V>> {}
    pub fn get_index(&self, index: usize) -> Option<(&K, &V)> {
        self.as_entries().get(index).map(Bucket::refs)
    }
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
