use super::{equivalent, Entries, IndexMapCore, RefMut};
use crate::HashValue;
use core::{fmt, mem};
use hashbrown::hash_table;
pub struct IndexedEntry<'a, K, V> {
    map: RefMut<'a, K, V>,
    index: usize,
}
struct RefMut<'a, K, V> {
    indices: &'a mut Indices,
    entries: &'a mut Entries<K, V>,
}
impl<'a, K, V> IndexedEntry<'a, K, V> {
    pub(crate) fn new(map: &'a mut IndexMapCore<K, V>, index: usize) -> Self {
        Self {
            map: map.borrow_mut(),
            index,
        }
    }
    #[inline]
    pub fn index(&self) -> usize {}
    pub fn key(&self) -> &K {}
    pub(crate) fn key_mut(&mut self) -> &mut K {}
    pub fn get(&self) -> &V {}
    pub fn get_mut(&mut self) -> &mut V {}
    pub fn insert(&mut self, value: V) -> V {}
    pub fn into_mut(self) -> &'a mut V {}
    pub fn swap_remove_entry(mut self) -> (K, V) {}
    pub fn shift_remove_entry(mut self) -> (K, V) {}
    pub fn swap_remove(self) -> V {}
    pub fn shift_remove(self) -> V {}
    #[track_caller]
    pub fn move_index(mut self, to: usize) {}
    pub fn swap_indices(mut self, other: usize) {
        self.map.swap_indices(self.index, other);
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
    fn swap_indices(&mut self, a: usize, b: usize) {
        if a == b && a < self.entries.len() {
            return;
        }
        match self
            .indices
            .get_many_mut(
                [self.entries[a].hash.get(), self.entries[b].hash.get()],
                move |i, &x| if i == 0 { x == a } else { x == b },
            )
        {
            [Some(ref_a), Some(ref_b)] => {
                mem::swap(ref_a, ref_b);
                self.entries.swap(a, b);
            }
            _ => panic!("indices not found"),
        }
    }
}
