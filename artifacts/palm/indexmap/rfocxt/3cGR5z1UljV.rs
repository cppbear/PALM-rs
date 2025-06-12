pub use self::core::raw_entry_v1::{self, RawEntryApiV1};
pub use self::core::{Entry, IndexedEntry, OccupiedEntry, VacantEntry};
pub use self::iter::{
    Drain, IntoIter, IntoKeys, IntoValues, Iter, IterMut, IterMut2, Keys, Splice, Values,
    ValuesMut,
};
pub use self::mutable::MutableEntryKey;
pub use self::mutable::MutableKeys;
pub use self::slice::Slice;
#[cfg(feature = "rayon")]
pub use crate::rayon::map as rayon;
use ::core::cmp::Ordering;
use ::core::fmt;
use ::core::hash::{BuildHasher, Hash, Hasher};
use ::core::mem;
use ::core::ops::{Index, IndexMut, RangeBounds};
use alloc::boxed::Box;
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::collections::hash_map::RandomState;
use self::core::IndexMapCore;
use crate::util::{third, try_simplify_range};
use crate::{
    Bucket, Entries, Equivalent, GetDisjointMutError, HashValue, TryReserveError,
};
pub trait MutableKeys: private::Sealed {
    type Key;
    type Value;
    fn get_full_mut2<Q>(
        &mut self,
        key: &Q,
    ) -> Option<(usize, &mut Self::Key, &mut Self::Value)>
    where
        Q: ?Sized + Hash + Equivalent<Self::Key>;
    fn get_index_mut2(
        &mut self,
        index: usize,
    ) -> Option<(&mut Self::Key, &mut Self::Value)>;
    fn iter_mut2(&mut self) -> IterMut2<'_, Self::Key, Self::Value>;
    fn retain2<F>(&mut self, keep: F)
    where
        F: FnMut(&mut Self::Key, &mut Self::Value) -> bool;
}
pub trait RawEntryApiV1<K, V, S>: private::Sealed {
    fn raw_entry_v1(&self) -> RawEntryBuilder<'_, K, V, S>;
    fn raw_entry_mut_v1(&mut self) -> RawEntryBuilderMut<'_, K, V, S>;
}
trait Entries {
    type Entry;
    fn into_entries(self) -> Vec<Self::Entry>;
    fn as_entries(&self) -> &[Self::Entry];
    fn as_entries_mut(&mut self) -> &mut [Self::Entry];
    fn with_entries<F>(&mut self, f: F)
    where
        F: FnOnce(&mut [Self::Entry]);
}
#[cfg(not(feature = "std"))]
pub struct IndexMap<K, V, S> {
    pub(crate) core: IndexMapCore<K, V>,
    hash_builder: S,
}
#[derive(Copy, Debug)]
struct Bucket<K, V> {
    hash: HashValue,
    key: K,
    value: V,
}
#[derive(Debug)]
pub(crate) struct IndexMapCore<K, V> {
    /// indices mapping from the entry hash to its index.
    indices: Indices,
    /// entries is a dense vec maintaining entry order.
    entries: Entries<K, V>,
}
impl<K, V, S> IndexMap<K, V, S>
where
    S: BuildHasher,
{
    pub(crate) fn hash<Q: ?Sized + Hash>(&self, key: &Q) -> HashValue {}
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {}
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        if let Some(i) = self.get_index_of(key) {
            let entry = &self.as_entries()[i];
            Some(&entry.value)
        } else {
            None
        }
    }
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {}
    pub fn get_full<Q>(&self, key: &Q) -> Option<(usize, &K, &V)>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {}
    pub fn get_index_of<Q>(&self, key: &Q) -> Option<usize>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {}
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {}
    pub fn get_full_mut<Q>(&mut self, key: &Q) -> Option<(usize, &K, &mut V)>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {}
    pub fn get_disjoint_mut<Q, const N: usize>(
        &mut self,
        keys: [&Q; N],
    ) -> [Option<&mut V>; N]
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {}
    #[deprecated(
        note = "`remove` disrupts the map order -- \
        use `swap_remove` or `shift_remove` for explicit behavior."
    )]
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {}
    #[deprecated(
        note = "`remove_entry` disrupts the map order -- \
        use `swap_remove_entry` or `shift_remove_entry` for explicit behavior."
    )]
    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {}
    pub fn swap_remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {}
    pub fn swap_remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {}
    pub fn swap_remove_full<Q>(&mut self, key: &Q) -> Option<(usize, K, V)>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {}
    pub fn shift_remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {}
    pub fn shift_remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {}
    pub fn shift_remove_full<Q>(&mut self, key: &Q) -> Option<(usize, K, V)>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {}
}
