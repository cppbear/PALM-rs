use super::{Entries, RefMut};
use crate::{Equivalent, HashValue, IndexMap};
use core::fmt;
use core::hash::{BuildHasher, Hash, Hasher};
use core::marker::PhantomData;
use core::mem;
use hashbrown::hash_table;
pub struct RawEntryBuilderMut<'a, K, V, S> {
    map: &'a mut IndexMap<K, V, S>,
}
struct RefMut<'a, K, V> {
    indices: &'a mut Indices,
    entries: &'a mut Entries<K, V>,
}
#[derive(Copy, Debug)]
struct Bucket<K, V> {
    hash: HashValue,
    key: K,
    value: V,
}
#[cfg(not(feature = "std"))]
pub struct IndexMap<K, V, S> {
    pub(crate) core: IndexMapCore<K, V>,
    hash_builder: S,
}
pub struct RawOccupiedEntryMut<'a, K, V, S> {
    entries: &'a mut Entries<K, V>,
    index: hash_table::OccupiedEntry<'a, usize>,
    hash_builder: PhantomData<&'a S>,
}
pub struct RawVacantEntryMut<'a, K, V, S> {
    map: RefMut<'a, K, V>,
    hash_builder: &'a S,
}
pub enum RawEntryMut<'a, K, V, S> {
    /// Existing slot with equivalent key.
    Occupied(RawOccupiedEntryMut<'a, K, V, S>),
    /// Vacant slot (no equivalent key in the map).
    Vacant(RawVacantEntryMut<'a, K, V, S>),
}
impl<'a, K, V, S> RawEntryBuilderMut<'a, K, V, S> {
    pub fn from_key<Q>(self, key: &Q) -> RawEntryMut<'a, K, V, S>
    where
        S: BuildHasher,
        Q: ?Sized + Hash + Equivalent<K>,
    {}
    pub fn from_key_hashed_nocheck<Q>(
        self,
        hash: u64,
        key: &Q,
    ) -> RawEntryMut<'a, K, V, S>
    where
        Q: ?Sized + Equivalent<K>,
    {}
    pub fn from_hash<F>(self, hash: u64, mut is_match: F) -> RawEntryMut<'a, K, V, S>
    where
        F: FnMut(&K) -> bool,
    {
        let ref_entries = &*self.map.core.entries;
        let eq = move |&i: &usize| is_match(&ref_entries[i].key);
        match self.map.core.indices.find_entry(hash, eq) {
            Ok(index) => {
                RawEntryMut::Occupied(RawOccupiedEntryMut {
                    entries: &mut self.map.core.entries,
                    index,
                    hash_builder: PhantomData,
                })
            }
            Err(absent) => {
                RawEntryMut::Vacant(RawVacantEntryMut {
                    map: RefMut::new(absent.into_table(), &mut self.map.core.entries),
                    hash_builder: &self.map.hash_builder,
                })
            }
        }
    }
}
impl<'a, K, V> RefMut<'a, K, V> {
    #[inline]
    fn new(indices: &'a mut Indices, entries: &'a mut Entries<K, V>) -> Self {
        Self { indices, entries }
    }
    #[inline]
    fn reserve_entries(&mut self, additional: usize) {}
    fn insert_unique(
        self,
        hash: HashValue,
        key: K,
        value: V,
    ) -> OccupiedEntry<'a, K, V> {}
    fn shift_insert_unique(&mut self, index: usize, hash: HashValue, key: K, value: V) {}
    fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {}
    fn shift_remove_finish(&mut self, index: usize) -> (K, V) {}
    fn swap_remove_index(&mut self, index: usize) -> Option<(K, V)> {}
    fn swap_remove_finish(&mut self, index: usize) -> (K, V) {}
    fn decrement_indices(&mut self, start: usize, end: usize) {}
    fn increment_indices(&mut self, start: usize, end: usize) {}
    #[track_caller]
    fn move_index(&mut self, from: usize, to: usize) {}
    #[track_caller]
    fn swap_indices(&mut self, a: usize, b: usize) {}
}
