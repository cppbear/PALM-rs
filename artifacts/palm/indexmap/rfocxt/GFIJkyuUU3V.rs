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
impl<K, V, S> fmt::Debug for RawVacantEntryMut<'_, K, V, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawVacantEntryMut").finish_non_exhaustive()
    }
}
