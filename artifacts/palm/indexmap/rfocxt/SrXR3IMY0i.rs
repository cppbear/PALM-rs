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
impl<K: fmt::Debug, V: fmt::Debug, S> fmt::Debug for RawEntryMut<'_, K, V, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tuple = f.debug_tuple("RawEntryMut");
        match self {
            Self::Vacant(v) => tuple.field(v),
            Self::Occupied(o) => tuple.field(o),
        };
        tuple.finish()
    }
}
