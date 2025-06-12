use super::{Entries, RefMut};
use crate::{Equivalent, HashValue, IndexMap};
use core::fmt;
use core::hash::{BuildHasher, Hash, Hasher};
use core::marker::PhantomData;
use core::mem;
use hashbrown::hash_table;
pub struct RawEntryBuilderMut<'a, K, V, S> {
    map: &'a mut IndexMap<K, V, S>,
}
#[cfg(not(feature = "std"))]
pub struct IndexMap<K, V, S> {
    pub(crate) core: IndexMapCore<K, V>,
    hash_builder: S,
}
pub enum RawEntryMut<'a, K, V, S> {
    /// Existing slot with equivalent key.
    Occupied(RawOccupiedEntryMut<'a, K, V, S>),
    /// Vacant slot (no equivalent key in the map).
    Vacant(RawVacantEntryMut<'a, K, V, S>),
}
impl<'a, K, V, S> RawEntryBuilderMut<'a, K, V, S> {
    pub fn from_key<Q>(self, key: &Q) -> RawEntryMut<'a, K, V, S>
    where
        S: BuildHasher,
        Q: ?Sized + Hash + Equivalent<K>,
    {}
    pub fn from_key_hashed_nocheck<Q>(
        self,
        hash: u64,
        key: &Q,
    ) -> RawEntryMut<'a, K, V, S>
    where
        Q: ?Sized + Equivalent<K>,
    {
        self.from_hash(hash, |k| Q::equivalent(key, k))
    }
    pub fn from_hash<F>(self, hash: u64, mut is_match: F) -> RawEntryMut<'a, K, V, S>
    where
        F: FnMut(&K) -> bool,
    {}
}
