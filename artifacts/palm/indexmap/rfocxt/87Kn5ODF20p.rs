use super::{Entries, RefMut};
use crate::{Equivalent, HashValue, IndexMap};
use core::fmt;
use core::hash::{BuildHasher, Hash, Hasher};
use core::marker::PhantomData;
use core::mem;
use hashbrown::hash_table;
pub struct RawOccupiedEntryMut<'a, K, V, S> {
    entries: &'a mut Entries<K, V>,
    index: hash_table::OccupiedEntry<'a, usize>,
    hash_builder: PhantomData<&'a S>,
}
pub struct RawVacantEntryMut<'a, K, V, S> {
    map: RefMut<'a, K, V>,
    hash_builder: &'a S,
}
pub enum RawEntryMut<'a, K, V, S> {
    /// Existing slot with equivalent key.
    Occupied(RawOccupiedEntryMut<'a, K, V, S>),
    /// Vacant slot (no equivalent key in the map).
    Vacant(RawVacantEntryMut<'a, K, V, S>),
}
impl<'a, K, V, S> RawEntryMut<'a, K, V, S> {
    #[inline]
    pub fn index(&self) -> usize {}
    pub fn or_insert(self, default_key: K, default_value: V) -> (&'a mut K, &'a mut V)
    where
        K: Hash,
        S: BuildHasher,
    {
        match self {
            Self::Occupied(entry) => entry.into_key_value_mut(),
            Self::Vacant(entry) => entry.insert(default_key, default_value),
        }
    }
    pub fn or_insert_with<F>(self, call: F) -> (&'a mut K, &'a mut V)
    where
        F: FnOnce() -> (K, V),
        K: Hash,
        S: BuildHasher,
    {}
    pub fn and_modify<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut K, &mut V),
    {
        if let Self::Occupied(entry) = &mut self {
            let (k, v) = entry.get_key_value_mut();
            f(k, v);
        }
        self
    }
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
    pub fn get_mut(&mut self) -> &mut V {}
    pub fn into_mut(self) -> &'a mut V {}
    pub fn get_key_value(&self) -> (&K, &V) {}
    pub fn get_key_value_mut(&mut self) -> (&mut K, &mut V) {}
    pub fn into_key_value_mut(self) -> (&'a mut K, &'a mut V) {
        let index = self.index();
        self.entries[index].muts()
    }
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
    pub fn move_index(self, to: usize) {}
    pub fn swap_indices(self, other: usize) {}
}
impl<'a, K, V, S> RawVacantEntryMut<'a, K, V, S> {
    pub fn index(&self) -> usize {}
    pub fn insert(self, key: K, value: V) -> (&'a mut K, &'a mut V)
    where
        K: Hash,
        S: BuildHasher,
    {
        let mut h = self.hash_builder.build_hasher();
        key.hash(&mut h);
        self.insert_hashed_nocheck(h.finish(), key, value)
    }
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
    ) -> (&'a mut K, &'a mut V) {}
}
