use super::{equivalent, Entries, IndexMapCore, RefMut};
use crate::HashValue;
use core::{fmt, mem};
use hashbrown::hash_table;
pub struct IndexedEntry<'a, K, V> {
    map: RefMut<'a, K, V>,
    index: usize,
}
#[derive(Debug)]
pub(crate) struct IndexMapCore<K, V> {
    /// indices mapping from the entry hash to its index.
    indices: Indices,
    /// entries is a dense vec maintaining entry order.
    entries: Entries<K, V>,
}
struct RefMut<'a, K, V> {
    indices: &'a mut Indices,
    entries: &'a mut Entries<K, V>,
}
impl<'a, K, V> IndexedEntry<'a, K, V> {
    pub(crate) fn new(map: &'a mut IndexMapCore<K, V>, index: usize) -> Self {
        Self {
            map: map.borrow_mut(),
            index,
        }
    }
    #[inline]
    pub fn index(&self) -> usize {}
    pub fn key(&self) -> &K {}
    pub(crate) fn key_mut(&mut self) -> &mut K {}
    pub fn get(&self) -> &V {}
    pub fn get_mut(&mut self) -> &mut V {}
    pub fn insert(&mut self, value: V) -> V {}
    pub fn into_mut(self) -> &'a mut V {}
    pub fn swap_remove_entry(mut self) -> (K, V) {}
    pub fn shift_remove_entry(mut self) -> (K, V) {}
    pub fn swap_remove(self) -> V {}
    pub fn shift_remove(self) -> V {}
    #[track_caller]
    pub fn move_index(mut self, to: usize) {}
    pub fn swap_indices(mut self, other: usize) {}
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
    fn borrow_mut(&mut self) -> RefMut<'_, K, V> {
        RefMut::new(&mut self.indices, &mut self.entries)
    }
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
    pub(crate) fn pop(&mut self) -> Option<(K, V)> {}
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
