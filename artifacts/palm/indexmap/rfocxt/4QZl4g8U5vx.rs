use super::{Entries, RefMut};
use crate::{Equivalent, HashValue, IndexMap};
use core::fmt;
use core::hash::{BuildHasher, Hash, Hasher};
use core::marker::PhantomData;
use core::mem;
use hashbrown::hash_table;
trait Entries {
    type Entry;
    fn into_entries(self) -> Vec<Self::Entry>;
    fn as_entries(&self) -> &[Self::Entry];
    fn as_entries_mut(&mut self) -> &mut [Self::Entry];
    fn with_entries<F>(&mut self, f: F)
    where
        F: FnOnce(&mut [Self::Entry]);
}
pub struct RawOccupiedEntryMut<'a, K, V, S> {
    entries: &'a mut Entries<K, V>,
    index: hash_table::OccupiedEntry<'a, usize>,
    hash_builder: PhantomData<&'a S>,
}
struct RefMut<'a, K, V> {
    indices: &'a mut Indices,
    entries: &'a mut Entries<K, V>,
}
pub struct OccupiedEntry<'a, K, V> {
    entries: &'a mut Entries<K, V>,
    index: hash_table::OccupiedEntry<'a, usize>,
}
impl<'a, K, V, S> RawOccupiedEntryMut<'a, K, V, S> {
    #[inline]
    pub fn index(&self) -> usize {
        *self.index.get()
    }
    #[inline]
    fn into_ref_mut(self) -> RefMut<'a, K, V> {
        RefMut::new(self.index.into_table(), self.entries)
    }
    pub fn key(&self) -> &K {}
    pub fn key_mut(&mut self) -> &mut K {}
    pub fn into_key(self) -> &'a mut K {}
    pub fn get(&self) -> &V {}
    pub fn get_mut(&mut self) -> &mut V {}
    pub fn into_mut(self) -> &'a mut V {}
    pub fn get_key_value(&self) -> (&K, &V) {}
    pub fn get_key_value_mut(&mut self) -> (&mut K, &mut V) {}
    pub fn into_key_value_mut(self) -> (&'a mut K, &'a mut V) {}
    pub fn insert(&mut self, value: V) -> V {}
    pub fn insert_key(&mut self, key: K) -> K {}
    #[deprecated(
        note = "`remove` disrupts the map order -- \
        use `swap_remove` or `shift_remove` for explicit behavior."
    )]
    pub fn remove(self) -> V {}
    pub fn swap_remove(self) -> V {}
    pub fn shift_remove(self) -> V {}
    #[deprecated(
        note = "`remove_entry` disrupts the map order -- \
        use `swap_remove_entry` or `shift_remove_entry` for explicit behavior."
    )]
    pub fn remove_entry(self) -> (K, V) {}
    pub fn swap_remove_entry(self) -> (K, V) {}
    pub fn shift_remove_entry(self) -> (K, V) {}
    pub fn move_index(self, to: usize) {
        let index = self.index();
        self.into_ref_mut().move_index(index, to);
    }
    pub fn swap_indices(self, other: usize) {}
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
    fn move_index(&mut self, from: usize, to: usize) {
        let from_hash = self.entries[from].hash;
        let _ = self.entries[to];
        if from != to {
            update_index(self.indices, from_hash, from, usize::MAX);
            if from < to {
                self.decrement_indices(from + 1, to + 1);
                self.entries[from..=to].rotate_left(1);
            } else if to < from {
                self.increment_indices(to, from);
                self.entries[to..=from].rotate_right(1);
            }
            update_index(self.indices, from_hash, usize::MAX, to);
        }
    }
    #[track_caller]
    fn swap_indices(&mut self, a: usize, b: usize) {}
}
