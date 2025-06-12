use super::{equivalent, Entries, IndexMapCore, RefMut};
use crate::HashValue;
use core::{fmt, mem};
use hashbrown::hash_table;
pub struct VacantEntry<'a, K, V> {
    map: RefMut<'a, K, V>,
    hash: HashValue,
    key: K,
}
#[derive(Clone, Copy, Debug, PartialEq)]
struct HashValue(usize);
struct RefMut<'a, K, V> {
    indices: &'a mut Indices,
    entries: &'a mut Entries<K, V>,
}
impl<K: fmt::Debug, V> fmt::Debug for VacantEntry<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("VacantEntry").field(self.key()).finish()
    }
}
impl<'a, K, V> VacantEntry<'a, K, V> {
    pub fn index(&self) -> usize {}
    pub fn key(&self) -> &K {
        &self.key
    }
    pub(crate) fn key_mut(&mut self) -> &mut K {}
    pub fn into_key(self) -> K {}
    pub fn insert(self, value: V) -> &'a mut V {}
    pub fn insert_entry(self, value: V) -> OccupiedEntry<'a, K, V> {}
    pub fn insert_sorted(self, value: V) -> (usize, &'a mut V)
    where
        K: Ord,
    {}
    pub fn shift_insert(mut self, index: usize, value: V) -> &'a mut V {}
}
