use super::{Entries, RefMut};
use crate::{Equivalent, HashValue, IndexMap};
use core::fmt;
use core::hash::{BuildHasher, Hash, Hasher};
use core::marker::PhantomData;
use core::mem;
use hashbrown::hash_table;
pub struct RawEntryBuilder<'a, K, V, S> {
    map: &'a IndexMap<K, V, S>,
}
#[cfg(not(feature = "std"))]
pub struct IndexMap<K, V, S> {
    pub(crate) core: IndexMapCore<K, V>,
    hash_builder: S,
}
impl<'a, K, V, S> RawEntryBuilder<'a, K, V, S> {
    pub fn from_key<Q>(self, key: &Q) -> Option<(&'a K, &'a V)>
    where
        S: BuildHasher,
        Q: ?Sized + Hash + Equivalent<K>,
    {
        self.map.get_key_value(key)
    }
    pub fn from_key_hashed_nocheck<Q>(self, hash: u64, key: &Q) -> Option<(&'a K, &'a V)>
    where
        Q: ?Sized + Equivalent<K>,
    {}
    pub fn from_hash<F>(self, hash: u64, is_match: F) -> Option<(&'a K, &'a V)>
    where
        F: FnMut(&K) -> bool,
    {}
    pub fn from_hash_full<F>(
        self,
        hash: u64,
        is_match: F,
    ) -> Option<(usize, &'a K, &'a V)>
    where
        F: FnMut(&K) -> bool,
    {}
    pub fn index_from_hash<F>(self, hash: u64, mut is_match: F) -> Option<usize>
    where
        F: FnMut(&K) -> bool,
    {}
}
