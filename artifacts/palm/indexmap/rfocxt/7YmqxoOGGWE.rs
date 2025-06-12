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
#[derive(Debug)]
pub(crate) struct IndexMapCore<K, V> {
    /// indices mapping from the entry hash to its index.
    indices: Indices,
    /// entries is a dense vec maintaining entry order.
    entries: Entries<K, V>,
}
#[derive(Clone, Copy, Debug, PartialEq)]
struct HashValue(usize);
#[derive(Copy, Debug)]
struct Bucket<K, V> {
    hash: HashValue,
    key: K,
    value: V,
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
    {}
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
    {
        match self.as_entries() {
            [x] if key.equivalent(&x.key) => {
                let (k, v) = self.core.pop()?;
                Some((0, k, v))
            }
            [_] | [] => None,
            _ => {
                let hash = self.hash(key);
                self.core.shift_remove_full(hash, key)
            }
        }
    }
}
impl<K, V> IndexMapCore<K, V> {
    const MAX_ENTRIES_CAPACITY: usize = (isize::MAX as usize)
        / mem::size_of::<Bucket<K, V>>();
    #[inline]
    pub(crate) const fn new() -> Self {
        IndexMapCore {
            indices: Indices::new(),
            entries: Vec::new(),
        }
    }
    #[inline]
    fn borrow_mut(&mut self) -> RefMut<'_, K, V> {}
    #[inline]
    pub(crate) fn with_capacity(n: usize) -> Self {
        IndexMapCore {
            indices: Indices::with_capacity(n),
            entries: Vec::with_capacity(n),
        }
    }
    #[inline]
    pub(crate) fn len(&self) -> usize {}
    #[inline]
    pub(crate) fn capacity(&self) -> usize {}
    pub(crate) fn clear(&mut self) {}
    pub(crate) fn truncate(&mut self, len: usize) {}
    #[track_caller]
    pub(crate) fn drain<R>(&mut self, range: R) -> vec::Drain<'_, Bucket<K, V>>
    where
        R: RangeBounds<usize>,
    {}
    #[cfg(feature = "rayon")]
    pub(crate) fn par_drain<R>(
        &mut self,
        range: R,
    ) -> rayon::vec::Drain<'_, Bucket<K, V>>
    where
        K: Send,
        V: Send,
        R: RangeBounds<usize>,
    {}
    #[track_caller]
    pub(crate) fn split_off(&mut self, at: usize) -> Self {
        let len = self.entries.len();
        assert!(
            at <= len,
            "index out of bounds: the len is {len} but the index is {at}. Expected index <= len"
        );
        self.erase_indices(at, self.entries.len());
        let entries = self.entries.split_off(at);
        let mut indices = Indices::with_capacity(entries.len());
        insert_bulk_no_grow(&mut indices, &entries);
        Self { indices, entries }
    }
    #[track_caller]
    pub(crate) fn split_splice<R>(
        &mut self,
        range: R,
    ) -> (Self, vec::IntoIter<Bucket<K, V>>)
    where
        R: RangeBounds<usize>,
    {
        let range = simplify_range(range, self.len());
        self.erase_indices(range.start, self.entries.len());
        let entries = self.entries.split_off(range.end);
        let drained = self.entries.split_off(range.start);
        let mut indices = Indices::with_capacity(entries.len());
        insert_bulk_no_grow(&mut indices, &entries);
        (Self { indices, entries }, drained.into_iter())
    }
    pub(crate) fn append_unchecked(&mut self, other: &mut Self) {}
    pub(crate) fn reserve(&mut self, additional: usize) {}
    pub(crate) fn reserve_exact(&mut self, additional: usize) {}
    pub(crate) fn try_reserve(
        &mut self,
        additional: usize,
    ) -> Result<(), TryReserveError> {}
    fn try_reserve_entries(&mut self, additional: usize) -> Result<(), TryReserveError> {}
    pub(crate) fn try_reserve_exact(
        &mut self,
        additional: usize,
    ) -> Result<(), TryReserveError> {}
    pub(crate) fn shrink_to(&mut self, min_capacity: usize) {}
    pub(crate) fn pop(&mut self) -> Option<(K, V)> {
        if let Some(entry) = self.entries.pop() {
            let last = self.entries.len();
            erase_index(&mut self.indices, entry.hash, last);
            Some((entry.key, entry.value))
        } else {
            None
        }
    }
    pub(crate) fn get_index_of<Q>(&self, hash: HashValue, key: &Q) -> Option<usize>
    where
        Q: ?Sized + Equivalent<K>,
    {}
    fn push_entry(&mut self, hash: HashValue, key: K, value: V) {}
    pub(crate) fn insert_full(
        &mut self,
        hash: HashValue,
        key: K,
        value: V,
    ) -> (usize, Option<V>)
    where
        K: Eq,
    {}
    pub(crate) fn replace_full(
        &mut self,
        hash: HashValue,
        key: K,
        value: V,
    ) -> (usize, Option<(K, V)>)
    where
        K: Eq,
    {}
    pub(crate) fn shift_remove_full<Q>(
        &mut self,
        hash: HashValue,
        key: &Q,
    ) -> Option<(usize, K, V)>
    where
        Q: ?Sized + Equivalent<K>,
    {}
    #[inline]
    pub(crate) fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {}
    #[inline]
    #[track_caller]
    pub(super) fn move_index(&mut self, from: usize, to: usize) {}
    #[inline]
    #[track_caller]
    pub(crate) fn swap_indices(&mut self, a: usize, b: usize) {}
    pub(crate) fn swap_remove_full<Q>(
        &mut self,
        hash: HashValue,
        key: &Q,
    ) -> Option<(usize, K, V)>
    where
        Q: ?Sized + Equivalent<K>,
    {}
    #[inline]
    pub(crate) fn swap_remove_index(&mut self, index: usize) -> Option<(K, V)> {}
    fn erase_indices(&mut self, start: usize, end: usize) {}
    pub(crate) fn retain_in_order<F>(&mut self, mut keep: F)
    where
        F: FnMut(&mut K, &mut V) -> bool,
    {}
    fn rebuild_hash_table(&mut self) {}
    pub(crate) fn reverse(&mut self) {}
}
