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
pub struct OccupiedEntry<'a, K, V> {
    entries: &'a mut Entries<K, V>,
    index: hash_table::OccupiedEntry<'a, usize>,
}
impl<'a, K, V, S> RawOccupiedEntryMut<'a, K, V, S> {
    #[inline]
    pub fn index(&self) -> usize {}
    #[inline]
    fn into_ref_mut(self) -> RefMut<'a, K, V> {}
    pub fn key(&self) -> &K {}
    pub fn key_mut(&mut self) -> &mut K {}
    pub fn into_key(self) -> &'a mut K {}
    pub fn get(&self) -> &V {}
    pub fn get_mut(&mut self) -> &mut V {
        let index = self.index();
        &mut self.entries[index].value
    }
    pub fn into_mut(self) -> &'a mut V {}
    pub fn get_key_value(&self) -> (&K, &V) {}
    pub fn get_key_value_mut(&mut self) -> (&mut K, &mut V) {}
    pub fn into_key_value_mut(self) -> (&'a mut K, &'a mut V) {}
    pub fn insert(&mut self, value: V) -> V {
        mem::replace(self.get_mut(), value)
    }
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
    pub fn move_index(self, to: usize) {}
    pub fn swap_indices(self, other: usize) {}
}
