use core::hash::{BuildHasher, Hash};
use super::{
    Bucket, Entries, Entry, Equivalent, IndexMap, IndexedEntry, IterMut2, OccupiedEntry,
    VacantEntry,
};
pub trait MutableEntryKey: private::Sealed {
    type Key;
    fn key_mut(&mut self) -> &mut Self::Key;
}
pub trait Sealed {}
pub struct IndexedEntry<'a, K, V> {
    map: RefMut<'a, K, V>,
    index: usize,
}
impl<K, V> MutableEntryKey for IndexedEntry<'_, K, V> {
    type Key = K;
    fn key_mut(&mut self) -> &mut Self::Key {
        self.key_mut()
    }
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
    pub(crate) fn key_mut(&mut self) -> &mut K {
        &mut self.map.entries[self.index].key
    }
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
    pub fn swap_indices(mut self, other: usize) {}
}
