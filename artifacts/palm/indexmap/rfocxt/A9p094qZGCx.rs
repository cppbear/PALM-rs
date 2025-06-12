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
    pub fn get_mut(&mut self) -> &mut V {
        &mut self.map.entries[self.index].value
    }
    pub fn insert(&mut self, value: V) -> V {
        mem::replace(self.get_mut(), value)
    }
    pub fn into_mut(self) -> &'a mut V {}
    pub fn swap_remove_entry(mut self) -> (K, V) {}
    pub fn shift_remove_entry(mut self) -> (K, V) {}
    pub fn swap_remove(self) -> V {}
    pub fn shift_remove(self) -> V {}
    #[track_caller]
    pub fn move_index(mut self, to: usize) {}
    pub fn swap_indices(mut self, other: usize) {}
}
