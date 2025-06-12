use super::{equivalent, Entries, IndexMapCore, RefMut};
use crate::HashValue;
use core::{fmt, mem};
use hashbrown::hash_table;
pub struct VacantEntry<'a, K, V> {
    map: RefMut<'a, K, V>,
    hash: HashValue,
    key: K,
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
#[derive(Clone, Copy, Debug, PartialEq)]
struct HashValue(usize);
impl<'a, K, V> VacantEntry<'a, K, V> {
    pub fn index(&self) -> usize {}
    pub fn key(&self) -> &K {}
    pub(crate) fn key_mut(&mut self) -> &mut K {}
    pub fn into_key(self) -> K {}
    pub fn insert(self, value: V) -> &'a mut V {}
    pub fn insert_entry(self, value: V) -> OccupiedEntry<'a, K, V> {}
    pub fn insert_sorted(self, value: V) -> (usize, &'a mut V)
    where
        K: Ord,
    {}
    pub fn shift_insert(mut self, index: usize, value: V) -> &'a mut V {
        self.map.shift_insert_unique(index, self.hash, self.key, value);
        &mut self.map.entries[index].value
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
