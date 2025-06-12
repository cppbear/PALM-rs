use super::{Entries, RefMut};
use crate::{Equivalent, HashValue, IndexMap};
use core::fmt;
use core::hash::{BuildHasher, Hash, Hasher};
use core::marker::PhantomData;
use core::mem;
use hashbrown::hash_table;
pub struct RawVacantEntryMut<'a, K, V, S> {
    map: RefMut<'a, K, V>,
    hash_builder: &'a S,
}
#[derive(Copy, Debug)]
struct Bucket<K, V> {
    hash: HashValue,
    key: K,
    value: V,
}
struct RefMut<'a, K, V> {
    indices: &'a mut Indices,
    entries: &'a mut Entries<K, V>,
}
#[derive(Clone, Copy, Debug, PartialEq)]
struct HashValue(usize);
impl<'a, K, V, S> RawVacantEntryMut<'a, K, V, S> {
    pub fn index(&self) -> usize {}
    pub fn insert(self, key: K, value: V) -> (&'a mut K, &'a mut V)
    where
        K: Hash,
        S: BuildHasher,
    {}
    pub fn insert_hashed_nocheck(
        self,
        hash: u64,
        key: K,
        value: V,
    ) -> (&'a mut K, &'a mut V) {}
    pub fn shift_insert(self, index: usize, key: K, value: V) -> (&'a mut K, &'a mut V)
    where
        K: Hash,
        S: BuildHasher,
    {}
    pub fn shift_insert_hashed_nocheck(
        mut self,
        index: usize,
        hash: u64,
        key: K,
        value: V,
    ) -> (&'a mut K, &'a mut V) {
        let hash = HashValue(hash as usize);
        self.map.shift_insert_unique(index, hash, key, value);
        self.map.entries[index].muts()
    }
}
impl<K, V> Bucket<K, V> {
    fn key_ref(&self) -> &K {}
    fn value_ref(&self) -> &V {}
    fn value_mut(&mut self) -> &mut V {}
    fn key(self) -> K {}
    fn value(self) -> V {}
    fn key_value(self) -> (K, V) {}
    fn refs(&self) -> (&K, &V) {}
    fn ref_mut(&mut self) -> (&K, &mut V) {}
    fn muts(&mut self) -> (&mut K, &mut V) {
        (&mut self.key, &mut self.value)
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
    fn shift_insert_unique(&mut self, index: usize, hash: HashValue, key: K, value: V) {
        let end = self.indices.len();
        assert!(index <= end);
        self.increment_indices(index, end);
        let entries = &*self.entries;
        self.indices
            .insert_unique(
                hash.get(),
                index,
                move |&i| {
                    debug_assert_ne!(i, index);
                    let i = if i < index { i } else { i - 1 };
                    entries[i].hash.get()
                },
            );
        if self.entries.len() == self.entries.capacity() {
            self.reserve_entries(1);
        }
        self.entries.insert(index, Bucket { hash, key, value });
    }
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
