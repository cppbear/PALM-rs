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
pub struct VacantEntry<'a, K, V> {
    map: RefMut<'a, K, V>,
    hash: HashValue,
    key: K,
}
impl<K, V> MutableEntryKey for VacantEntry<'_, K, V> {
    type Key = K;
    fn key_mut(&mut self) -> &mut Self::Key {
        self.key_mut()
    }
}
impl<'a, K, V> VacantEntry<'a, K, V> {
    pub fn index(&self) -> usize {}
    pub fn key(&self) -> &K {}
    pub(crate) fn key_mut(&mut self) -> &mut K {
        &mut self.key
    }
    pub fn into_key(self) -> K {}
    pub fn insert(self, value: V) -> &'a mut V {}
    pub fn insert_entry(self, value: V) -> OccupiedEntry<'a, K, V> {}
    pub fn insert_sorted(self, value: V) -> (usize, &'a mut V)
    where
        K: Ord,
    {}
    pub fn shift_insert(mut self, index: usize, value: V) -> &'a mut V {}
}
