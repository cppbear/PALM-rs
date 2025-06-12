use super::{Entries, RefMut};
use crate::{Equivalent, HashValue, IndexMap};
use core::fmt;
use core::hash::{BuildHasher, Hash, Hasher};
use core::marker::PhantomData;
use core::mem;
use hashbrown::hash_table;
pub struct RawVacantEntryMut<'a, K, V, S> {
    map: RefMut<'a, K, V>,
    hash_builder: &'a S,
}
struct RefMut<'a, K, V> {
    indices: &'a mut Indices,
    entries: &'a mut Entries<K, V>,
}
impl<'a, K, V, S> RawVacantEntryMut<'a, K, V, S> {
    pub fn index(&self) -> usize {
        self.map.indices.len()
    }
    pub fn insert(self, key: K, value: V) -> (&'a mut K, &'a mut V)
    where
        K: Hash,
        S: BuildHasher,
    {}
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
