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
pub struct OccupiedEntry<'a, K, V> {
    entries: &'a mut Entries<K, V>,
    index: hash_table::OccupiedEntry<'a, usize>,
}
pub struct VacantEntry<'a, K, V> {
    map: RefMut<'a, K, V>,
    hash: HashValue,
    key: K,
}
#[derive(Debug)]
pub(crate) struct IndexMapCore<K, V> {
    /// indices mapping from the entry hash to its index.
    indices: Indices,
    /// entries is a dense vec maintaining entry order.
    entries: Entries<K, V>,
}
pub enum Entry<'a, K, V> {
    /// Existing slot with equivalent key.
    Occupied(OccupiedEntry<'a, K, V>),
    /// Vacant slot (no equivalent key in the map).
    Vacant(VacantEntry<'a, K, V>),
}
impl<K, V, S> IndexMap<K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {}
    pub fn insert_full(&mut self, key: K, value: V) -> (usize, Option<V>) {}
    pub fn insert_sorted(&mut self, key: K, value: V) -> (usize, Option<V>)
    where
        K: Ord,
    {}
    #[track_caller]
    pub fn insert_before(
        &mut self,
        mut index: usize,
        key: K,
        value: V,
    ) -> (usize, Option<V>) {
        let len = self.len();
        assert!(
            index <= len,
            "index out of bounds: the len is {len} but the index is {index}. Expected index <= len"
        );
        match self.entry(key) {
            Entry::Occupied(mut entry) => {
                if index > entry.index() {
                    index -= 1;
                }
                let old = mem::replace(entry.get_mut(), value);
                entry.move_index(index);
                (index, Some(old))
            }
            Entry::Vacant(entry) => {
                entry.shift_insert(index, value);
                (index, None)
            }
        }
    }
    #[track_caller]
    pub fn shift_insert(&mut self, index: usize, key: K, value: V) -> Option<V> {}
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        let hash = self.hash(&key);
        self.core.entry(hash, key)
    }
    #[track_caller]
    pub fn splice<R, I>(
        &mut self,
        range: R,
        replace_with: I,
    ) -> Splice<'_, I::IntoIter, K, V, S>
    where
        R: RangeBounds<usize>,
        I: IntoIterator<Item = (K, V)>,
    {}
    pub fn append<S2>(&mut self, other: &mut IndexMap<K, V, S2>) {}
}
impl<'a, K, V> OccupiedEntry<'a, K, V> {
    pub(crate) fn new(
        entries: &'a mut Entries<K, V>,
        index: hash_table::OccupiedEntry<'a, usize>,
    ) -> Self {
        Self { entries, index }
    }
    #[inline]
    pub fn index(&self) -> usize {
        *self.index.get()
    }
    #[inline]
    fn into_ref_mut(self) -> RefMut<'a, K, V> {}
    pub fn key(&self) -> &K {}
    pub(crate) fn key_mut(&mut self) -> &mut K {}
    pub fn get(&self) -> &V {}
    pub fn get_mut(&mut self) -> &mut V {
        let index = self.index();
        &mut self.entries[index].value
    }
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
    pub fn remove_entry(self) -> (K, V) {}
    pub fn swap_remove_entry(self) -> (K, V) {}
    pub fn shift_remove_entry(self) -> (K, V) {}
    #[track_caller]
    pub fn move_index(self, to: usize) {
        let index = self.index();
        self.into_ref_mut().move_index(index, to);
    }
    pub fn swap_indices(self, other: usize) {}
}
impl<K, V, S> IndexMap<K, V, S> {
    #[inline]
    pub fn with_capacity_and_hasher(n: usize, hash_builder: S) -> Self {
        if n == 0 {
            Self::with_hasher(hash_builder)
        } else {
            IndexMap {
                core: IndexMapCore::with_capacity(n),
                hash_builder,
            }
        }
    }
    pub const fn with_hasher(hash_builder: S) -> Self {
        IndexMap {
            core: IndexMapCore::new(),
            hash_builder,
        }
    }
    pub fn capacity(&self) -> usize {}
    pub fn hasher(&self) -> &S {}
    #[inline]
    pub fn len(&self) -> usize {
        self.core.len()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {}
    pub fn iter(&self) -> Iter<'_, K, V> {}
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {}
    pub fn keys(&self) -> Keys<'_, K, V> {}
    pub fn into_keys(self) -> IntoKeys<K, V> {}
    pub fn values(&self) -> Values<'_, K, V> {}
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {}
    pub fn into_values(self) -> IntoValues<K, V> {}
    pub fn clear(&mut self) {}
    pub fn truncate(&mut self, len: usize) {}
    #[track_caller]
    pub fn drain<R>(&mut self, range: R) -> Drain<'_, K, V>
    where
        R: RangeBounds<usize>,
    {}
    #[track_caller]
    pub fn split_off(&mut self, at: usize) -> Self
    where
        S: Clone,
    {
        Self {
            core: self.core.split_off(at),
            hash_builder: self.hash_builder.clone(),
        }
    }
    pub fn reserve(&mut self, additional: usize) {}
    pub fn reserve_exact(&mut self, additional: usize) {}
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {}
    pub fn try_reserve_exact(
        &mut self,
        additional: usize,
    ) -> Result<(), TryReserveError> {}
    pub fn shrink_to_fit(&mut self) {}
    pub fn shrink_to(&mut self, min_capacity: usize) {}
}
impl<'a, K, V> VacantEntry<'a, K, V> {
    pub fn index(&self) -> usize {}
    pub fn key(&self) -> &K {}
    pub(crate) fn key_mut(&mut self) -> &mut K {}
    pub fn into_key(self) -> K {}
    pub fn insert(self, value: V) -> &'a mut V {}
    pub fn insert_entry(self, value: V) -> OccupiedEntry<'a, K, V> {}
    pub fn insert_sorted(self, value: V) -> (usize, &'a mut V)
    where
        K: Ord,
    {}
    pub fn shift_insert(mut self, index: usize, value: V) -> &'a mut V {
        self.map.shift_insert_unique(index, self.hash, self.key, value);
        &mut self.map.entries[index].value
    }
}
