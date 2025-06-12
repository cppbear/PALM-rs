use super::{equivalent, Entries, IndexMapCore, RefMut};
use crate::HashValue;
use core::{fmt, mem};
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
pub struct OccupiedEntry<'a, K, V> {
    entries: &'a mut Entries<K, V>,
    index: hash_table::OccupiedEntry<'a, usize>,
}
impl<'a, K, V> OccupiedEntry<'a, K, V> {
    pub(crate) fn new(
        entries: &'a mut Entries<K, V>,
        index: hash_table::OccupiedEntry<'a, usize>,
    ) -> Self {
        Self { entries, index }
    }
    #[inline]
    pub fn index(&self) -> usize {}
    #[inline]
    fn into_ref_mut(self) -> RefMut<'a, K, V> {}
    pub fn key(&self) -> &K {}
    pub(crate) fn key_mut(&mut self) -> &mut K {}
    pub fn get(&self) -> &V {}
    pub fn get_mut(&mut self) -> &mut V {}
    pub fn into_mut(self) -> &'a mut V {}
    pub(super) fn into_muts(self) -> (&'a mut K, &'a mut V) {}
    pub fn insert(&mut self, value: V) -> V {}
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
    pub fn remove_entry(self) -> (K, V) {
        self.swap_remove_entry()
    }
    pub fn swap_remove_entry(self) -> (K, V) {
        let (index, entry) = self.index.remove();
        RefMut::new(entry.into_table(), self.entries).swap_remove_finish(index)
    }
    pub fn shift_remove_entry(self) -> (K, V) {}
    #[track_caller]
    pub fn move_index(self, to: usize) {}
    pub fn swap_indices(self, other: usize) {}
}
