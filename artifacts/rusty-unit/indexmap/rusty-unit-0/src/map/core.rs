//! This is the core implementation that doesn't depend on the hasher at all.
//!
//! The methods of `IndexMapCore` don't use any Hash properties of K.
//!
//! It's cleaner to separate them out, then the compiler checks that we are not
//! using Hash at all in these methods.
//!
//! However, we should probably not let this show in the public API or docs.

mod entry;

pub mod raw_entry_v1;

use hashbrown::hash_table;

use crate::vec::{self, Vec};
use crate::TryReserveError;
use core::mem;
use core::ops::RangeBounds;

use crate::util::simplify_range;
use crate::{Bucket, Equivalent, HashValue};

type Indices = hash_table::HashTable<usize>;
type Entries<K, V> = Vec<Bucket<K, V>>;

pub use entry::{Entry, IndexedEntry, OccupiedEntry, VacantEntry};

/// Core of the map that does not depend on S
#[derive(Debug)]
pub(crate) struct IndexMapCore<K, V> {
    /// indices mapping from the entry hash to its index.
    indices: Indices,
    /// entries is a dense vec maintaining entry order.
    entries: Entries<K, V>,
}

/// Mutable references to the parts of an `IndexMapCore`.
///
/// When using `HashTable::find_entry`, that takes hold of `&mut indices`, so we have to borrow our
/// `&mut entries` separately, and there's no way to go back to a `&mut IndexMapCore`. So this type
/// is used to implement methods on the split references, and `IndexMapCore` can also call those to
/// avoid duplication.
struct RefMut<'a, K, V> {
    indices: &'a mut Indices,
    entries: &'a mut Entries<K, V>,
}

#[inline(always)]
fn get_hash<K, V>(entries: &[Bucket<K, V>]) -> impl Fn(&usize) -> u64 + '_ {
    move |&i| entries[i].hash.get()
}

#[inline]
fn equivalent<'a, K, V, Q: ?Sized + Equivalent<K>>(
    key: &'a Q,
    entries: &'a [Bucket<K, V>],
) -> impl Fn(&usize) -> bool + 'a {
    move |&i| Q::equivalent(key, &entries[i].key)
}

#[inline]
fn erase_index(table: &mut Indices, hash: HashValue, index: usize) {
    if let Ok(entry) = table.find_entry(hash.get(), move |&i| i == index) {
        entry.remove();
    } else if cfg!(debug_assertions) {
        panic!("index not found");
    }
}

#[inline]
fn update_index(table: &mut Indices, hash: HashValue, old: usize, new: usize) {
    let index = table
        .find_mut(hash.get(), move |&i| i == old)
        .expect("index not found");
    *index = new;
}

/// Inserts many entries into the indices table without reallocating,
/// and without regard for duplication.
///
/// ***Panics*** if there is not sufficient capacity already.
fn insert_bulk_no_grow<K, V>(indices: &mut Indices, entries: &[Bucket<K, V>]) {
    assert!(indices.capacity() - indices.len() >= entries.len());
    for entry in entries {
        indices.insert_unique(entry.hash.get(), indices.len(), |_| unreachable!());
    }
}

impl<K, V> Clone for IndexMapCore<K, V>
where
    K: Clone,
    V: Clone,
{
    fn clone(&self) -> Self {
        let mut new = Self::new();
        new.clone_from(self);
        new
    }

    fn clone_from(&mut self, other: &Self) {
        self.indices.clone_from(&other.indices);
        if self.entries.capacity() < other.entries.len() {
            // If we must resize, match the indices capacity.
            let additional = other.entries.len() - self.entries.len();
            self.borrow_mut().reserve_entries(additional);
        }
        self.entries.clone_from(&other.entries);
    }
}

impl<K, V> crate::Entries for IndexMapCore<K, V> {
    type Entry = Bucket<K, V>;

    #[inline]
    fn into_entries(self) -> Vec<Self::Entry> {
        self.entries
    }

    #[inline]
    fn as_entries(&self) -> &[Self::Entry] {
        &self.entries
    }

    #[inline]
    fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
        &mut self.entries
    }

    fn with_entries<F>(&mut self, f: F)
    where
        F: FnOnce(&mut [Self::Entry]),
    {
        f(&mut self.entries);
        self.rebuild_hash_table();
    }
}

impl<K, V> IndexMapCore<K, V> {
    /// The maximum capacity before the `entries` allocation would exceed `isize::MAX`.
    const MAX_ENTRIES_CAPACITY: usize = (isize::MAX as usize) / mem::size_of::<Bucket<K, V>>();

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
    pub(crate) fn len(&self) -> usize {
        self.indices.len()
    }

    #[inline]
    pub(crate) fn capacity(&self) -> usize {
        Ord::min(self.indices.capacity(), self.entries.capacity())
    }

    pub(crate) fn clear(&mut self) {
        self.indices.clear();
        self.entries.clear();
    }

    pub(crate) fn truncate(&mut self, len: usize) {
        if len < self.len() {
            self.erase_indices(len, self.entries.len());
            self.entries.truncate(len);
        }
    }

    #[track_caller]
    pub(crate) fn drain<R>(&mut self, range: R) -> vec::Drain<'_, Bucket<K, V>>
    where
        R: RangeBounds<usize>,
    {
        let range = simplify_range(range, self.entries.len());
        self.erase_indices(range.start, range.end);
        self.entries.drain(range)
    }

    #[cfg(feature = "rayon")]
    pub(crate) fn par_drain<R>(&mut self, range: R) -> rayon::vec::Drain<'_, Bucket<K, V>>
    where
        K: Send,
        V: Send,
        R: RangeBounds<usize>,
    {
        use rayon::iter::ParallelDrainRange;
        let range = simplify_range(range, self.entries.len());
        self.erase_indices(range.start, range.end);
        self.entries.par_drain(range)
    }

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
    pub(crate) fn split_splice<R>(&mut self, range: R) -> (Self, vec::IntoIter<Bucket<K, V>>)
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

    /// Append from another map without checking whether items already exist.
    pub(crate) fn append_unchecked(&mut self, other: &mut Self) {
        self.reserve(other.len());
        insert_bulk_no_grow(&mut self.indices, &other.entries);
        self.entries.append(&mut other.entries);
        other.indices.clear();
    }

    /// Reserve capacity for `additional` more key-value pairs.
    pub(crate) fn reserve(&mut self, additional: usize) {
        self.indices.reserve(additional, get_hash(&self.entries));
        // Only grow entries if necessary, since we also round up capacity.
        if additional > self.entries.capacity() - self.entries.len() {
            self.borrow_mut().reserve_entries(additional);
        }
    }

    /// Reserve capacity for `additional` more key-value pairs, without over-allocating.
    pub(crate) fn reserve_exact(&mut self, additional: usize) {
        self.indices.reserve(additional, get_hash(&self.entries));
        self.entries.reserve_exact(additional);
    }

    /// Try to reserve capacity for `additional` more key-value pairs.
    pub(crate) fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.indices
            .try_reserve(additional, get_hash(&self.entries))
            .map_err(TryReserveError::from_hashbrown)?;
        // Only grow entries if necessary, since we also round up capacity.
        if additional > self.entries.capacity() - self.entries.len() {
            self.try_reserve_entries(additional)
        } else {
            Ok(())
        }
    }

    /// Try to reserve entries capacity, rounded up to match the indices
    fn try_reserve_entries(&mut self, additional: usize) -> Result<(), TryReserveError> {
        // Use a soft-limit on the maximum capacity, but if the caller explicitly
        // requested more, do it and let them have the resulting error.
        let new_capacity = Ord::min(self.indices.capacity(), Self::MAX_ENTRIES_CAPACITY);
        let try_add = new_capacity - self.entries.len();
        if try_add > additional && self.entries.try_reserve_exact(try_add).is_ok() {
            return Ok(());
        }
        self.entries
            .try_reserve_exact(additional)
            .map_err(TryReserveError::from_alloc)
    }

    /// Try to reserve capacity for `additional` more key-value pairs, without over-allocating.
    pub(crate) fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.indices
            .try_reserve(additional, get_hash(&self.entries))
            .map_err(TryReserveError::from_hashbrown)?;
        self.entries
            .try_reserve_exact(additional)
            .map_err(TryReserveError::from_alloc)
    }

    /// Shrink the capacity of the map with a lower bound
    pub(crate) fn shrink_to(&mut self, min_capacity: usize) {
        self.indices
            .shrink_to(min_capacity, get_hash(&self.entries));
        self.entries.shrink_to(min_capacity);
    }

    /// Remove the last key-value pair
    pub(crate) fn pop(&mut self) -> Option<(K, V)> {
        if let Some(entry) = self.entries.pop() {
            let last = self.entries.len();
            erase_index(&mut self.indices, entry.hash, last);
            Some((entry.key, entry.value))
        } else {
            None
        }
    }

    /// Return the index in `entries` where an equivalent key can be found
    pub(crate) fn get_index_of<Q>(&self, hash: HashValue, key: &Q) -> Option<usize>
    where
        Q: ?Sized + Equivalent<K>,
    {
        let eq = equivalent(key, &self.entries);
        self.indices.find(hash.get(), eq).copied()
    }

    /// Append a key-value pair to `entries`,
    /// *without* checking whether it already exists.
    fn push_entry(&mut self, hash: HashValue, key: K, value: V) {
        if self.entries.len() == self.entries.capacity() {
            // Reserve our own capacity synced to the indices,
            // rather than letting `Vec::push` just double it.
            self.borrow_mut().reserve_entries(1);
        }
        self.entries.push(Bucket { hash, key, value });
    }

    pub(crate) fn insert_full(&mut self, hash: HashValue, key: K, value: V) -> (usize, Option<V>)
    where
        K: Eq,
    {
        let eq = equivalent(&key, &self.entries);
        let hasher = get_hash(&self.entries);
        match self.indices.entry(hash.get(), eq, hasher) {
            hash_table::Entry::Occupied(entry) => {
                let i = *entry.get();
                (i, Some(mem::replace(&mut self.entries[i].value, value)))
            }
            hash_table::Entry::Vacant(entry) => {
                let i = self.entries.len();
                entry.insert(i);
                self.push_entry(hash, key, value);
                debug_assert_eq!(self.indices.len(), self.entries.len());
                (i, None)
            }
        }
    }

    /// Same as `insert_full`, except it also replaces the key
    pub(crate) fn replace_full(
        &mut self,
        hash: HashValue,
        key: K,
        value: V,
    ) -> (usize, Option<(K, V)>)
    where
        K: Eq,
    {
        let eq = equivalent(&key, &self.entries);
        let hasher = get_hash(&self.entries);
        match self.indices.entry(hash.get(), eq, hasher) {
            hash_table::Entry::Occupied(entry) => {
                let i = *entry.get();
                let entry = &mut self.entries[i];
                let kv = (
                    mem::replace(&mut entry.key, key),
                    mem::replace(&mut entry.value, value),
                );
                (i, Some(kv))
            }
            hash_table::Entry::Vacant(entry) => {
                let i = self.entries.len();
                entry.insert(i);
                self.push_entry(hash, key, value);
                debug_assert_eq!(self.indices.len(), self.entries.len());
                (i, None)
            }
        }
    }

    /// Remove an entry by shifting all entries that follow it
    pub(crate) fn shift_remove_full<Q>(&mut self, hash: HashValue, key: &Q) -> Option<(usize, K, V)>
    where
        Q: ?Sized + Equivalent<K>,
    {
        let eq = equivalent(key, &self.entries);
        match self.indices.find_entry(hash.get(), eq) {
            Ok(entry) => {
                let (index, _) = entry.remove();
                let (key, value) = self.borrow_mut().shift_remove_finish(index);
                Some((index, key, value))
            }
            Err(_) => None,
        }
    }

    /// Remove an entry by shifting all entries that follow it
    #[inline]
    pub(crate) fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        self.borrow_mut().shift_remove_index(index)
    }

    #[inline]
    #[track_caller]
    pub(super) fn move_index(&mut self, from: usize, to: usize) {
        self.borrow_mut().move_index(from, to);
    }

    #[inline]
    #[track_caller]
    pub(crate) fn swap_indices(&mut self, a: usize, b: usize) {
        self.borrow_mut().swap_indices(a, b);
    }

    /// Remove an entry by swapping it with the last
    pub(crate) fn swap_remove_full<Q>(&mut self, hash: HashValue, key: &Q) -> Option<(usize, K, V)>
    where
        Q: ?Sized + Equivalent<K>,
    {
        let eq = equivalent(key, &self.entries);
        match self.indices.find_entry(hash.get(), eq) {
            Ok(entry) => {
                let (index, _) = entry.remove();
                let (key, value) = self.borrow_mut().swap_remove_finish(index);
                Some((index, key, value))
            }
            Err(_) => None,
        }
    }

    /// Remove an entry by swapping it with the last
    #[inline]
    pub(crate) fn swap_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        self.borrow_mut().swap_remove_index(index)
    }

    /// Erase `start..end` from `indices`, and shift `end..` indices down to `start..`
    ///
    /// All of these items should still be at their original location in `entries`.
    /// This is used by `drain`, which will let `Vec::drain` do the work on `entries`.
    fn erase_indices(&mut self, start: usize, end: usize) {
        let (init, shifted_entries) = self.entries.split_at(end);
        let (start_entries, erased_entries) = init.split_at(start);

        let erased = erased_entries.len();
        let shifted = shifted_entries.len();
        let half_capacity = self.indices.capacity() / 2;

        // Use a heuristic between different strategies
        if erased == 0 {
            // Degenerate case, nothing to do
        } else if start + shifted < half_capacity && start < erased {
            // Reinsert everything, as there are few kept indices
            self.indices.clear();

            // Reinsert stable indices, then shifted indices
            insert_bulk_no_grow(&mut self.indices, start_entries);
            insert_bulk_no_grow(&mut self.indices, shifted_entries);
        } else if erased + shifted < half_capacity {
            // Find each affected index, as there are few to adjust

            // Find erased indices
            for (i, entry) in (start..).zip(erased_entries) {
                erase_index(&mut self.indices, entry.hash, i);
            }

            // Find shifted indices
            for ((new, old), entry) in (start..).zip(end..).zip(shifted_entries) {
                update_index(&mut self.indices, entry.hash, old, new);
            }
        } else {
            // Sweep the whole table for adjustments
            let offset = end - start;
            self.indices.retain(move |i| {
                if *i >= end {
                    *i -= offset;
                    true
                } else {
                    *i < start
                }
            });
        }

        debug_assert_eq!(self.indices.len(), start + shifted);
    }

    pub(crate) fn retain_in_order<F>(&mut self, mut keep: F)
    where
        F: FnMut(&mut K, &mut V) -> bool,
    {
        self.entries
            .retain_mut(|entry| keep(&mut entry.key, &mut entry.value));
        if self.entries.len() < self.indices.len() {
            self.rebuild_hash_table();
        }
    }

    fn rebuild_hash_table(&mut self) {
        self.indices.clear();
        insert_bulk_no_grow(&mut self.indices, &self.entries);
    }

    pub(crate) fn reverse(&mut self) {
        self.entries.reverse();

        // No need to save hash indices, can easily calculate what they should
        // be, given that this is an in-place reversal.
        let len = self.entries.len();
        for i in &mut self.indices {
            *i = len - *i - 1;
        }
    }
}

/// Reserve entries capacity, rounded up to match the indices (via `try_capacity`).
fn reserve_entries<K, V>(entries: &mut Entries<K, V>, additional: usize, try_capacity: usize) {
    // Use a soft-limit on the maximum capacity, but if the caller explicitly
    // requested more, do it and let them have the resulting panic.
    let try_capacity = try_capacity.min(IndexMapCore::<K, V>::MAX_ENTRIES_CAPACITY);
    let try_add = try_capacity - entries.len();
    if try_add > additional && entries.try_reserve_exact(try_add).is_ok() {
        return;
    }
    entries.reserve_exact(additional);
}

impl<'a, K, V> RefMut<'a, K, V> {
    #[inline]
    fn new(indices: &'a mut Indices, entries: &'a mut Entries<K, V>) -> Self {
        Self { indices, entries }
    }

    /// Reserve entries capacity, rounded up to match the indices
    #[inline]
    fn reserve_entries(&mut self, additional: usize) {
        reserve_entries(self.entries, additional, self.indices.capacity());
    }

    /// Insert a key-value pair in `entries`,
    /// *without* checking whether it already exists.
    fn insert_unique(self, hash: HashValue, key: K, value: V) -> OccupiedEntry<'a, K, V> {
        let i = self.indices.len();
        debug_assert_eq!(i, self.entries.len());
        let entry = self
            .indices
            .insert_unique(hash.get(), i, get_hash(self.entries));
        if self.entries.len() == self.entries.capacity() {
            // We can't call `indices.capacity()` while this `entry` has borrowed it, so we'll have
            // to amortize growth on our own. It's still an improvement over the basic `Vec::push`
            // doubling though, since we also consider `MAX_ENTRIES_CAPACITY`.
            reserve_entries(self.entries, 1, 2 * self.entries.capacity());
        }
        self.entries.push(Bucket { hash, key, value });
        OccupiedEntry::new(self.entries, entry)
    }

    /// Insert a key-value pair in `entries` at a particular index,
    /// *without* checking whether it already exists.
    fn shift_insert_unique(&mut self, index: usize, hash: HashValue, key: K, value: V) {
        let end = self.indices.len();
        assert!(index <= end);
        // Increment others first so we don't have duplicate indices.
        self.increment_indices(index, end);
        let entries = &*self.entries;
        self.indices.insert_unique(hash.get(), index, move |&i| {
            // Adjust for the incremented indices to find hashes.
            debug_assert_ne!(i, index);
            let i = if i < index { i } else { i - 1 };
            entries[i].hash.get()
        });
        if self.entries.len() == self.entries.capacity() {
            // Reserve our own capacity synced to the indices,
            // rather than letting `Vec::insert` just double it.
            self.reserve_entries(1);
        }
        self.entries.insert(index, Bucket { hash, key, value });
    }

    /// Remove an entry by shifting all entries that follow it
    fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        match self.entries.get(index) {
            Some(entry) => {
                erase_index(self.indices, entry.hash, index);
                Some(self.shift_remove_finish(index))
            }
            None => None,
        }
    }

    /// Remove an entry by shifting all entries that follow it
    ///
    /// The index should already be removed from `self.indices`.
    fn shift_remove_finish(&mut self, index: usize) -> (K, V) {
        // Correct indices that point to the entries that followed the removed entry.
        self.decrement_indices(index + 1, self.entries.len());

        // Use Vec::remove to actually remove the entry.
        let entry = self.entries.remove(index);
        (entry.key, entry.value)
    }

    /// Remove an entry by swapping it with the last
    fn swap_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        match self.entries.get(index) {
            Some(entry) => {
                erase_index(self.indices, entry.hash, index);
                Some(self.swap_remove_finish(index))
            }
            None => None,
        }
    }

    /// Finish removing an entry by swapping it with the last
    ///
    /// The index should already be removed from `self.indices`.
    fn swap_remove_finish(&mut self, index: usize) -> (K, V) {
        // use swap_remove, but then we need to update the index that points
        // to the other entry that has to move
        let entry = self.entries.swap_remove(index);

        // correct index that points to the entry that had to swap places
        if let Some(entry) = self.entries.get(index) {
            // was not last element
            // examine new element in `index` and find it in indices
            let last = self.entries.len();
            update_index(self.indices, entry.hash, last, index);
        }

        (entry.key, entry.value)
    }

    /// Decrement all indices in the range `start..end`.
    ///
    /// The index `start - 1` should not exist in `self.indices`.
    /// All entries should still be in their original positions.
    fn decrement_indices(&mut self, start: usize, end: usize) {
        // Use a heuristic between a full sweep vs. a `find()` for every shifted item.
        let shifted_entries = &self.entries[start..end];
        if shifted_entries.len() > self.indices.capacity() / 2 {
            // Shift all indices in range.
            for i in &mut *self.indices {
                if start <= *i && *i < end {
                    *i -= 1;
                }
            }
        } else {
            // Find each entry in range to shift its index.
            for (i, entry) in (start..end).zip(shifted_entries) {
                update_index(self.indices, entry.hash, i, i - 1);
            }
        }
    }

    /// Increment all indices in the range `start..end`.
    ///
    /// The index `end` should not exist in `self.indices`.
    /// All entries should still be in their original positions.
    fn increment_indices(&mut self, start: usize, end: usize) {
        // Use a heuristic between a full sweep vs. a `find()` for every shifted item.
        let shifted_entries = &self.entries[start..end];
        if shifted_entries.len() > self.indices.capacity() / 2 {
            // Shift all indices in range.
            for i in &mut *self.indices {
                if start <= *i && *i < end {
                    *i += 1;
                }
            }
        } else {
            // Find each entry in range to shift its index, updated in reverse so
            // we never have duplicated indices that might have a hash collision.
            for (i, entry) in (start..end).zip(shifted_entries).rev() {
                update_index(self.indices, entry.hash, i, i + 1);
            }
        }
    }

    #[track_caller]
    fn move_index(&mut self, from: usize, to: usize) {
        let from_hash = self.entries[from].hash;
        let _ = self.entries[to]; // explicit bounds check
        if from != to {
            // Use a sentinel index so other indices don't collide.
            update_index(self.indices, from_hash, from, usize::MAX);

            // Update all other indices and rotate the entry positions.
            if from < to {
                self.decrement_indices(from + 1, to + 1);
                self.entries[from..=to].rotate_left(1);
            } else if to < from {
                self.increment_indices(to, from);
                self.entries[to..=from].rotate_right(1);
            }

            // Change the sentinel index to its final position.
            update_index(self.indices, from_hash, usize::MAX, to);
        }
    }

    #[track_caller]
    fn swap_indices(&mut self, a: usize, b: usize) {
        // If they're equal and in-bounds, there's nothing to do.
        if a == b && a < self.entries.len() {
            return;
        }

        // We'll get a "nice" bounds-check from indexing `entries`,
        // and then we expect to find it in the table as well.
        match self.indices.get_many_mut(
            [self.entries[a].hash.get(), self.entries[b].hash.get()],
            move |i, &x| if i == 0 { x == a } else { x == b },
        ) {
            [Some(ref_a), Some(ref_b)] => {
                mem::swap(ref_a, ref_b);
                self.entries.swap(a, b);
            }
            _ => panic!("indices not found"),
        }
    }
}

#[test]
fn assert_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<IndexMapCore<i32, i32>>();
    assert_send_sync::<Entry<'_, i32, i32>>();
    assert_send_sync::<IndexedEntry<'_, i32, i32>>();
    assert_send_sync::<raw_entry_v1::RawEntryMut<'_, i32, i32, ()>>();
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::iter::ExactSizeIterator;
	use std::default::Default;
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use std::iter::Iterator;
	use std::ops::Index;
	use Entries;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1() {
    rusty_monitor::set_test_id(1);
    let mut isize_0: isize = -6942isize;
    let mut isize_1: isize = -3237isize;
    let mut iter_0: crate::map::iter::Iter<isize, isize> = crate::map::iter::Iter::default();
    let mut iter_0_ref_0: &crate::map::iter::Iter<isize, isize> = &mut iter_0;
    let mut usize_0: usize = 9152usize;
    let mut usize_1: usize = 6342usize;
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_1);
    let mut indexmapcore_0_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_0;
    let mut isize_2: isize = 8500isize;
    let mut isize_3: isize = 21067isize;
    let mut itermut_0: crate::map::iter::IterMut<isize, isize> = crate::map::iter::IterMut::default();
    let mut valuesmut_0: crate::map::iter::ValuesMut<isize, isize> = crate::map::iter::ValuesMut::default();
    let mut valuesmut_0_ref_0: &mut crate::map::iter::ValuesMut<isize, isize> = &mut valuesmut_0;
    let mut intokeys_0: crate::map::iter::IntoKeys<isize, isize> = crate::map::iter::IntoKeys::default();
    let mut values_0: crate::map::iter::Values<isize, isize> = crate::map::iter::Values::default();
    let mut isize_4: isize = -881isize;
    let mut usize_2: usize = 2342usize;
    let mut isize_5: isize = -7731isize;
    let mut isize_6: isize = 2859isize;
    let mut indexmapcore_1: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_1_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_1;
    let mut bucket_slice_0: &mut [crate::Bucket<isize, isize>] = crate::map::core::IndexMapCore::as_entries_mut(indexmapcore_1_ref_0);
    let mut option_0: std::option::Option<&isize> = crate::map::iter::Values::last(values_0);
    let mut isize_7: &isize = std::option::Option::unwrap(option_0);
    let mut option_1: std::option::Option<isize> = crate::map::iter::IntoKeys::last(intokeys_0);
    let mut isize_8: isize = std::option::Option::unwrap(option_1);
    let mut option_2: std::option::Option<&mut isize> = crate::map::iter::ValuesMut::next(valuesmut_0_ref_0);
    let mut isize_9: &mut isize = std::option::Option::unwrap(option_2);
    let mut usize_3: usize = crate::map::iter::IterMut::count(itermut_0);
    let mut result_0: std::result::Result<(), crate::TryReserveError> = crate::map::core::IndexMapCore::try_reserve_entries(indexmapcore_0_ref_0, usize_0);
    let mut tuple_0: (usize, std::option::Option<usize>) = crate::map::iter::Iter::size_hint(iter_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2() {
    rusty_monitor::set_test_id(2);
    let mut itermut2_0: crate::map::iter::IterMut2<isize, isize> = crate::map::iter::IterMut2::default();
    let mut itermut2_0_ref_0: &crate::map::iter::IterMut2<isize, isize> = &mut itermut2_0;
    let mut usize_0: usize = 1928usize;
    let mut usize_1: usize = 4498usize;
    let mut usize_2: usize = 773usize;
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_2);
    let mut indexmapcore_0_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_0;
    let mut usize_3: usize = 4023usize;
    let mut usize_4: usize = 56usize;
    let mut intoiter_0: crate::set::iter::IntoIter<isize> = crate::set::iter::IntoIter::default();
    let mut intoiter_0_ref_0: &mut crate::set::iter::IntoIter<isize> = &mut intoiter_0;
    let mut isize_0: isize = -8251isize;
    let mut isize_1: isize = 17836isize;
    let mut usize_5: usize = 1016usize;
    let mut usize_6: usize = 8698usize;
    let mut indexmapcore_1: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_6);
    let mut indexmapcore_1_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_1;
    let mut usize_7: usize = 9416usize;
    let mut isize_2: isize = -7050isize;
    let mut isize_3: isize = 2760isize;
    let mut u64_0: u64 = 6071u64;
    let mut isize_4: isize = -5949isize;
    let mut isize_4_ref_0: &isize = &mut isize_4;
    let mut option_0: std::option::Option<(isize, isize)> = crate::map::core::IndexMapCore::swap_remove_index(indexmapcore_1_ref_0, usize_5);
    let mut keys_0: crate::map::iter::Keys<isize, isize> = crate::map::iter::Keys::default();
    let mut option_1: std::option::Option<isize> = crate::set::iter::IntoIter::nth(intoiter_0_ref_0, usize_4);
    crate::map::core::IndexMapCore::erase_indices(indexmapcore_0_ref_0, usize_1, usize_0);
    let mut keys_0_ref_0: &crate::map::iter::Keys<isize, isize> = &mut keys_0;
    let mut tuple_0: (usize, std::option::Option<usize>) = crate::map::iter::Keys::size_hint(keys_0_ref_0);
    let mut tuple_1: (usize, std::option::Option<usize>) = crate::map::iter::IterMut2::size_hint(itermut2_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3() {
    rusty_monitor::set_test_id(3);
    let mut intoiter_0: crate::map::iter::IntoIter<isize, isize> = crate::map::iter::IntoIter::default();
    let mut intoiter_0_ref_0: &crate::map::iter::IntoIter<isize, isize> = &mut intoiter_0;
    let mut usize_0: usize = 7042usize;
    let mut valuesmut_0: crate::map::iter::ValuesMut<isize, isize> = crate::map::iter::ValuesMut::default();
    let mut valuesmut_0_ref_0: &mut crate::map::iter::ValuesMut<isize, isize> = &mut valuesmut_0;
    let mut intovalues_0: crate::map::iter::IntoValues<isize, std::result::Result<(), crate::TryReserveError>> = crate::map::iter::IntoValues::default();
    let mut intovalues_0_ref_0: &crate::map::iter::IntoValues<isize, std::result::Result<(), crate::TryReserveError>> = &mut intovalues_0;
    let mut u128_0: u128 = 467u128;
    let mut isize_0: isize = -6694isize;
    let mut iter_0: crate::map::iter::Iter<isize, isize> = crate::map::iter::Iter::default();
    let mut iter_0_ref_0: &mut crate::map::iter::Iter<isize, isize> = &mut iter_0;
    let mut isize_1: isize = 2686isize;
    let mut getdisjointmuterror_0: GetDisjointMutError = crate::GetDisjointMutError::OverlappingIndices;
    let mut getdisjointmuterror_0_ref_0: &GetDisjointMutError = &mut getdisjointmuterror_0;
    let mut iter_1: crate::set::iter::Iter<isize> = crate::set::iter::Iter::default();
    let mut iter_1_ref_0: &mut crate::set::iter::Iter<isize> = &mut iter_1;
    let mut usize_1: usize = 8957usize;
    let mut keys_0: crate::map::iter::Keys<isize, isize> = crate::map::iter::Keys::default();
    let mut keys_0_ref_0: &crate::map::iter::Keys<isize, isize> = &mut keys_0;
    let mut getdisjointmuterror_1: GetDisjointMutError = crate::GetDisjointMutError::IndexOutOfBounds;
    let mut intokeys_0: crate::map::iter::IntoKeys<isize, isize> = crate::map::iter::IntoKeys::default();
    let mut isize_2: &isize = crate::map::iter::Keys::index(keys_0_ref_0, usize_1);
    let mut option_0: std::option::Option<&isize> = crate::set::iter::Iter::next(iter_1_ref_0);
    let mut getdisjointmuterror_2: GetDisjointMutError = crate::GetDisjointMutError::clone(getdisjointmuterror_0_ref_0);
    let mut option_1: std::option::Option<(&isize, &isize)> = crate::map::iter::Iter::next(iter_0_ref_0);
    let mut option_2: std::option::Option<&mut isize> = crate::map::iter::ValuesMut::next(valuesmut_0_ref_0);
    let mut intokeys_0_ref_0: &crate::map::iter::IntoKeys<isize, isize> = &mut intokeys_0;
    let mut tuple_0: (usize, std::option::Option<usize>) = crate::map::iter::IntoKeys::size_hint(intokeys_0_ref_0);
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_0);
    let mut slice_0: &crate::map::slice::Slice<isize, isize> = crate::map::iter::IntoIter::as_slice(intoiter_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4() {
    rusty_monitor::set_test_id(4);
    let mut usize_0: usize = 437usize;
    let mut valuesmut_0: crate::map::iter::ValuesMut<isize, isize> = crate::map::iter::ValuesMut::default();
    let mut intoiter_0: crate::map::iter::IntoIter<isize, isize> = crate::map::iter::IntoIter::default();
    let mut intoiter_1: crate::map::iter::IntoIter<isize, isize> = crate::map::iter::IntoIter::default();
    let mut intoiter_1_ref_0: &crate::map::iter::IntoIter<isize, isize> = &mut intoiter_1;
    let mut intoiter_2: crate::map::iter::IntoIter<isize, isize> = crate::map::iter::IntoIter::default();
    let mut keys_0: crate::map::iter::Keys<isize, isize> = crate::map::iter::Keys::default();
    let mut keys_0_ref_0: &crate::map::iter::Keys<isize, isize> = &mut keys_0;
    let mut keys_1: crate::map::iter::Keys<isize, isize> = crate::map::iter::Keys::default();
    let mut keys_1_ref_0: &mut crate::map::iter::Keys<isize, isize> = &mut keys_1;
    let mut intoiter_3: crate::map::iter::IntoIter<isize, isize> = crate::map::iter::IntoIter::default();
    let mut iter_0: crate::set::iter::Iter<isize> = crate::set::iter::Iter::default();
    let mut isize_0: isize = 5398isize;
    let mut isize_1: isize = -16831isize;
    let mut isize_2: isize = 11468isize;
    let mut isize_3: isize = -4006isize;
    let mut usize_1: usize = 284usize;
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_1);
    let mut indexmapcore_0_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_0;
    let mut option_0: std::option::Option<&isize> = crate::set::iter::Iter::last(iter_0);
    let mut option_1: std::option::Option<(isize, isize)> = crate::map::iter::IntoIter::last(intoiter_3);
    let mut option_2: std::option::Option<&isize> = crate::map::iter::Keys::next(keys_1_ref_0);
    let mut tuple_0: (usize, std::option::Option<usize>) = crate::map::iter::Keys::size_hint(keys_0_ref_0);
    let mut option_3: std::option::Option<(isize, isize)> = crate::map::iter::IntoIter::last(intoiter_2);
    let mut usize_2: usize = crate::map::iter::IntoIter::len(intoiter_1_ref_0);
    let mut intoiter_4: crate::map::iter::IntoIter<isize, isize> = crate::map::iter::IntoIter::default();
    let mut option_4: std::option::Option<(isize, isize)> = crate::map::iter::IntoIter::last(intoiter_0);
    let mut tuple_1: (isize, isize) = std::option::Option::unwrap(option_1);
    let mut option_5: std::option::Option<&mut isize> = crate::map::iter::ValuesMut::last(valuesmut_0);
    let mut intoiter_4_ref_0: &mut crate::map::iter::IntoIter<isize, isize> = &mut intoiter_4;
    let mut option_6: std::option::Option<(isize, isize)> = crate::map::iter::IntoIter::nth(intoiter_4_ref_0, usize_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5() {
    rusty_monitor::set_test_id(5);
    let mut usize_0: usize = 9980usize;
    let mut itermut2_0: crate::map::iter::IterMut2<isize, isize> = crate::map::iter::IterMut2::default();
    let mut itermut2_0_ref_0: &mut crate::map::iter::IterMut2<isize, isize> = &mut itermut2_0;
    let mut isize_0: isize = 25526isize;
    let mut isize_1: isize = 3977isize;
    let mut getdisjointmuterror_0: GetDisjointMutError = crate::GetDisjointMutError::IndexOutOfBounds;
    let mut getdisjointmuterror_0_ref_0: &GetDisjointMutError = &mut getdisjointmuterror_0;
    let mut usize_1: usize = 1707usize;
    let mut usize_2: usize = 3692usize;
    let mut usize_3: usize = 2758usize;
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_3);
    let mut indexmapcore_0_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_0;
    let mut intokeys_0: crate::map::iter::IntoKeys<isize, isize> = crate::map::iter::IntoKeys::default();
    let mut intokeys_0_ref_0: &crate::map::iter::IntoKeys<isize, isize> = &mut intokeys_0;
    let mut isize_2: isize = -3852isize;
    let mut isize_3: isize = 6886isize;
    let mut u64_0: u64 = 5951u64;
    let mut usize_4: usize = 6002usize;
    let mut intovalues_0: crate::map::iter::IntoValues<isize, isize> = crate::map::iter::IntoValues::default();
    let mut intovalues_0_ref_0: &crate::map::iter::IntoValues<isize, isize> = &mut intovalues_0;
    let mut isize_4: isize = -17743isize;
    let mut usize_5: usize = 1493usize;
    let mut indexmapcore_1: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_5);
    let mut indexmapcore_1_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_1;
    let mut refmut_0: crate::map::core::RefMut<isize, isize> = crate::map::core::IndexMapCore::borrow_mut(indexmapcore_1_ref_0);
    let mut usize_6: usize = 3399usize;
    let mut usize_7: usize = crate::map::iter::IntoValues::len(intovalues_0_ref_0);
    let mut tuple_0: (usize, std::option::Option<usize>) = crate::map::iter::IntoKeys::size_hint(intokeys_0_ref_0);
    crate::map::core::IndexMapCore::erase_indices(indexmapcore_0_ref_0, usize_2, usize_1);
    let mut getdisjointmuterror_1: GetDisjointMutError = crate::GetDisjointMutError::clone(getdisjointmuterror_0_ref_0);
    let mut option_0: std::option::Option<(&mut isize, &mut isize)> = crate::map::iter::IterMut2::nth(itermut2_0_ref_0, usize_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8() {
    rusty_monitor::set_test_id(8);
    let mut intovalues_0: crate::map::iter::IntoValues<isize, isize> = crate::map::iter::IntoValues::default();
    let mut intovalues_0_ref_0: &mut crate::map::iter::IntoValues<isize, isize> = &mut intovalues_0;
    let mut isize_0: isize = 10129isize;
    let mut isize_1: isize = 28519isize;
    let mut isize_2: isize = -8683isize;
    let mut iter_0: crate::map::iter::Iter<isize, isize> = crate::map::iter::Iter::default();
    let mut usize_0: usize = 214usize;
    let mut iter_1: crate::map::iter::Iter<isize, isize> = crate::map::iter::Iter::default();
    let mut iter_1_ref_0: &mut crate::map::iter::Iter<isize, isize> = &mut iter_1;
    let mut usize_1: usize = 7453usize;
    let mut itermut2_0: crate::map::iter::IterMut2<isize, isize> = crate::map::iter::IterMut2::default();
    let mut itermut2_0_ref_0: &mut crate::map::iter::IterMut2<isize, isize> = &mut itermut2_0;
    let mut isize_3: isize = 8912isize;
    let mut isize_4: isize = 11286isize;
    let mut usize_2: usize = 8272usize;
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_0_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_0;
    let mut keys_0: crate::map::iter::Keys<std::string::String, isize> = crate::map::iter::Keys::default();
    let mut keys_0_ref_0: &crate::map::iter::Keys<std::string::String, isize> = &mut keys_0;
    let mut valuesmut_0: crate::map::iter::ValuesMut<isize, isize> = crate::map::iter::ValuesMut::default();
    let mut valuesmut_0_ref_0: &crate::map::iter::ValuesMut<isize, isize> = &mut valuesmut_0;
    let mut usize_3: usize = crate::map::iter::ValuesMut::len(valuesmut_0_ref_0);
    let mut option_0: std::option::Option<(isize, isize)> = crate::map::core::IndexMapCore::shift_remove_index(indexmapcore_0_ref_0, usize_2);
    let mut tuple_0: (isize, isize) = std::option::Option::unwrap(option_0);
    let mut intokeys_0: crate::map::iter::IntoKeys<isize, isize> = crate::map::iter::IntoKeys::default();
    let mut intokeys_0_ref_0: &crate::map::iter::IntoKeys<isize, isize> = &mut intokeys_0;
    let mut usize_4: usize = crate::map::iter::IntoKeys::len(intokeys_0_ref_0);
    let mut option_1: std::option::Option<(&mut isize, &mut isize)> = crate::map::iter::IterMut2::nth(itermut2_0_ref_0, usize_1);
    let mut option_2: std::option::Option<(&isize, &isize)> = crate::map::iter::Iter::nth(iter_1_ref_0, usize_0);
    let mut option_3: std::option::Option<(&isize, &isize)> = crate::map::iter::Iter::last(iter_0);
    let mut tuple_1: (&isize, &isize) = std::option::Option::unwrap(option_2);
    let mut option_4: std::option::Option<isize> = crate::map::iter::IntoValues::next(intovalues_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_10() {
    rusty_monitor::set_test_id(10);
    let mut iter_0: crate::set::iter::Iter<isize> = crate::set::iter::Iter::default();
    let mut usize_0: usize = 5093usize;
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_0_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_0;
    let mut itermut2_0: crate::map::iter::IterMut2<isize, isize> = crate::map::iter::IterMut2::default();
    let mut isize_0: isize = -11592isize;
    let mut isize_1: isize = 3752isize;
    let mut usize_1: usize = 5300usize;
    let mut usize_2: usize = 7981usize;
    let mut indexmapcore_1: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_1_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_1;
    let mut intoiter_0: crate::set::iter::IntoIter<isize> = crate::set::iter::IntoIter::default();
    let mut intoiter_0_ref_0: &crate::set::iter::IntoIter<isize> = &mut intoiter_0;
    let mut usize_3: usize = 6104usize;
    let mut indexmapcore_2: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_2_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_2;
    let mut usize_4: usize = 5800usize;
    let mut values_0: crate::map::iter::Values<isize, isize> = crate::map::iter::Values::default();
    let mut values_0_ref_0: &crate::map::iter::Values<isize, isize> = &mut values_0;
    let mut usize_5: usize = crate::map::iter::Values::len(values_0_ref_0);
    let mut result_0: std::result::Result<(), crate::TryReserveError> = crate::map::core::IndexMapCore::try_reserve_exact(indexmapcore_2_ref_0, usize_3);
    let mut tuple_0: () = std::result::Result::unwrap(result_0);
    let mut tuple_1: (usize, std::option::Option<usize>) = crate::set::iter::IntoIter::size_hint(intoiter_0_ref_0);
    let mut option_0: std::option::Option<(isize, isize)> = crate::map::core::IndexMapCore::shift_remove_index(indexmapcore_1_ref_0, usize_2);
    let mut slice_0: &mut crate::map::slice::Slice<isize, isize> = crate::map::iter::IterMut2::into_slice(itermut2_0);
    let mut keys_0: crate::map::iter::Keys<isize, isize> = crate::map::iter::Keys::default();
    let mut keys_0_ref_0: &crate::map::iter::Keys<isize, isize> = &mut keys_0;
    let mut tuple_2: (usize, std::option::Option<usize>) = crate::map::iter::Keys::size_hint(keys_0_ref_0);
    crate::map::core::IndexMapCore::shrink_to(indexmapcore_0_ref_0, usize_0);
    let mut option_1: std::option::Option<&isize> = crate::set::iter::Iter::last(iter_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_12() {
    rusty_monitor::set_test_id(12);
    let mut intoiter_0: crate::map::iter::IntoIter<isize, isize> = crate::map::iter::IntoIter::default();
    let mut intoiter_0_ref_0: &crate::map::iter::IntoIter<isize, isize> = &mut intoiter_0;
    let mut isize_0: isize = -417isize;
    let mut isize_1: isize = -12010isize;
    let mut u64_0: u64 = 3624u64;
    let mut isize_2: isize = 3274isize;
    let mut isize_3: isize = 50isize;
    let mut isize_4: isize = 883isize;
    let mut isize_5: isize = -18448isize;
    let mut usize_0: usize = 5544usize;
    let mut usize_1: usize = 6218usize;
    let mut usize_2: usize = 9156usize;
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_2);
    let mut indexmapcore_0_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_0;
    let mut isize_6: isize = 19111isize;
    let mut usize_3: usize = 3603usize;
    let mut indexmapcore_1: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_3);
    let mut indexmapcore_1_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_1;
    let mut iter_0: crate::map::iter::Iter<std::string::String, std::result::Result<(), crate::TryReserveError>> = crate::map::iter::Iter::default();
    let mut iter_0_ref_0: &crate::map::iter::Iter<std::string::String, std::result::Result<(), crate::TryReserveError>> = &mut iter_0;
    let mut values_0: crate::map::iter::Values<isize, isize> = crate::map::iter::Values::default();
    let mut values_0_ref_0: &crate::map::iter::Values<isize, isize> = &mut values_0;
    let mut usize_4: usize = crate::map::iter::Values::len(values_0_ref_0);
    crate::map::core::IndexMapCore::swap_indices(indexmapcore_0_ref_0, usize_1, usize_0);
    let mut intokeys_0: crate::map::iter::IntoKeys<isize, isize> = crate::map::iter::IntoKeys::default();
    let mut intokeys_0_ref_0: &crate::map::iter::IntoKeys<isize, isize> = &mut intokeys_0;
    let mut usize_5: usize = crate::map::iter::IntoKeys::len(intokeys_0_ref_0);
    let mut values_1: crate::map::iter::Values<isize, isize> = crate::map::iter::Values::default();
    let mut option_0: std::option::Option<&isize> = crate::map::iter::Values::last(values_1);
    let mut usize_6: usize = crate::map::iter::IntoIter::len(intoiter_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_13() {
    rusty_monitor::set_test_id(13);
    let mut usize_0: usize = 8873usize;
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_0_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_0;
    let mut usize_1: usize = 6740usize;
    let mut indexmapcore_1: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_1);
    let mut indexmapcore_1_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_1;
    let mut isize_0: isize = -2571isize;
    let mut isize_1: isize = -748isize;
    let mut usize_2: usize = 664usize;
    let mut usize_3: usize = 2433usize;
    let mut isize_2: isize = 3589isize;
    let mut isize_3: isize = 46isize;
    let mut isize_4: isize = 10451isize;
    let mut isize_5: isize = -1155isize;
    let mut isize_6: isize = -9635isize;
    let mut usize_4: usize = 3588usize;
    let mut itermut2_0: crate::map::iter::IterMut2<isize, isize> = crate::map::iter::IterMut2::default();
    let mut tuple_0: () = ();
    let mut result_0: std::result::Result<(), crate::TryReserveError> = std::result::Result::Ok(tuple_0);
    let mut usize_5: usize = 2744usize;
    let mut usize_6: usize = 1831usize;
    let mut intokeys_0: crate::map::iter::IntoKeys<std::string::String, isize> = crate::map::iter::IntoKeys::default();
    let mut intokeys_0_ref_0: &crate::map::iter::IntoKeys<std::string::String, isize> = &mut intokeys_0;
    let mut isize_7: isize = -4456isize;
    let mut isize_8: isize = 25816isize;
    let mut isize_9: isize = -1426isize;
    let mut isize_10: isize = -3465isize;
    let mut usize_7: usize = crate::map::iter::IterMut2::count(itermut2_0);
    let mut tuple_1: () = std::result::Result::unwrap(result_0);
    crate::map::core::IndexMapCore::clear(indexmapcore_1_ref_0);
    let mut indexmapcore_2: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::split_off(indexmapcore_0_ref_0, usize_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_14() {
    rusty_monitor::set_test_id(14);
    let mut values_0: crate::map::iter::Values<isize, isize> = crate::map::iter::Values::default();
    let mut usize_0: usize = 5328usize;
    let mut isize_0: isize = -13610isize;
    let mut isize_1: isize = -3263isize;
    let mut usize_1: usize = 7815usize;
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_0_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_0;
    let mut usize_2: usize = 4257usize;
    let mut indexmapcore_1: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_1_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_1;
    let mut usize_3: usize = 5984usize;
    let mut values_1: crate::map::iter::Values<isize, isize> = crate::map::iter::Values::default();
    let mut values_1_ref_0: &mut crate::map::iter::Values<isize, isize> = &mut values_1;
    let mut intoiter_0: crate::map::iter::IntoIter<isize, isize> = crate::map::iter::IntoIter::default();
    let mut intoiter_0_ref_0: &crate::map::iter::IntoIter<isize, isize> = &mut intoiter_0;
    let mut itermut_0: crate::map::iter::IterMut<isize, isize> = crate::map::iter::IterMut::default();
    let mut itermut_0_ref_0: &mut crate::map::iter::IterMut<isize, isize> = &mut itermut_0;
    let mut usize_4: usize = 9792usize;
    let mut indexmapcore_2: crate::map::core::IndexMapCore<i32, u128> = crate::map::core::IndexMapCore::with_capacity(usize_4);
    let mut indexmapcore_2_ref_0: &crate::map::core::IndexMapCore<i32, u128> = &mut indexmapcore_2;
    let mut usize_5: usize = 4592usize;
    let mut intoiter_1: crate::map::iter::IntoIter<isize, isize> = crate::map::iter::IntoIter::default();
    let mut intoiter_1_ref_0: &crate::map::iter::IntoIter<isize, isize> = &mut intoiter_1;
    let mut slice_0: &crate::map::slice::Slice<isize, isize> = crate::map::iter::IntoIter::as_slice(intoiter_1_ref_0);
    let mut indexmapcore_3: crate::map::core::IndexMapCore<i32, u128> = crate::map::core::IndexMapCore::clone(indexmapcore_2_ref_0);
    let mut option_0: std::option::Option<(&isize, &mut isize)> = crate::map::iter::IterMut::next(itermut_0_ref_0);
    let mut tuple_0: (usize, std::option::Option<usize>) = crate::map::iter::IntoIter::size_hint(intoiter_0_ref_0);
    let mut option_1: std::option::Option<&isize> = crate::map::iter::Values::next(values_1_ref_0);
    crate::map::core::IndexMapCore::shrink_to(indexmapcore_1_ref_0, usize_2);
    let mut result_0: std::result::Result<(), crate::TryReserveError> = crate::map::core::IndexMapCore::try_reserve_entries(indexmapcore_0_ref_0, usize_1);
    let mut option_2: std::option::Option<&isize> = crate::map::iter::Values::last(values_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_17() {
    rusty_monitor::set_test_id(17);
    let mut values_0: crate::map::iter::Values<isize, isize> = crate::map::iter::Values::default();
    let mut values_0_ref_0: &mut crate::map::iter::Values<isize, isize> = &mut values_0;
    let mut values_1: crate::map::iter::Values<isize, isize> = crate::map::iter::Values::default();
    let mut values_1_ref_0: &crate::map::iter::Values<isize, isize> = &mut values_1;
    let mut isize_0: isize = -6990isize;
    let mut isize_1: isize = 1150isize;
    let mut isize_2: isize = -3746isize;
    let mut valuesmut_0: crate::map::iter::ValuesMut<isize, isize> = crate::map::iter::ValuesMut::default();
    let mut valuesmut_0_ref_0: &crate::map::iter::ValuesMut<isize, isize> = &mut valuesmut_0;
    let mut isize_3: isize = 11838isize;
    let mut isize_4: isize = -9360isize;
    let mut isize_5: isize = 3573isize;
    let mut usize_0: usize = 3276usize;
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_0_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_0;
    let mut usize_1: usize = 5718usize;
    let mut iter_0: crate::set::iter::Iter<isize> = crate::set::iter::Iter::default();
    let mut iter_0_ref_0: &crate::set::iter::Iter<isize> = &mut iter_0;
    let mut usize_2: usize = 4607usize;
    let mut indexmapcore_1: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_2);
    let mut indexmapcore_1_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_1;
    let mut keys_0: crate::map::iter::Keys<isize, isize> = crate::map::iter::Keys::default();
    let mut usize_3: usize = crate::map::iter::Keys::count(keys_0);
    crate::map::core::IndexMapCore::reverse(indexmapcore_1_ref_0);
    let mut usize_4: usize = crate::set::iter::Iter::len(iter_0_ref_0);
    let mut option_0: std::option::Option<(isize, isize)> = crate::map::core::IndexMapCore::swap_remove_index(indexmapcore_0_ref_0, usize_0);
    let mut tuple_0: (isize, isize) = std::option::Option::unwrap(option_0);
    let mut tuple_1: (usize, std::option::Option<usize>) = crate::map::iter::ValuesMut::size_hint(valuesmut_0_ref_0);
    let mut usize_5: usize = crate::map::iter::Values::len(values_1_ref_0);
    let mut option_1: std::option::Option<&isize> = crate::map::iter::Values::next(values_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_18() {
    rusty_monitor::set_test_id(18);
    let mut usize_0: usize = 294usize;
    let mut usize_1: usize = 4110usize;
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_1);
    let mut indexmapcore_0_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_0;
    let mut result_0: std::result::Result<(), crate::TryReserveError> = crate::map::core::IndexMapCore::try_reserve_exact(indexmapcore_0_ref_0, usize_0);
    let mut usize_2: usize = 8554usize;
    let mut indexmapcore_1: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_1_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_1;
    let mut result_1: std::result::Result<(), crate::TryReserveError> = crate::map::core::IndexMapCore::try_reserve(indexmapcore_1_ref_0, usize_2);
    let mut values_0: crate::map::iter::Values<isize, std::result::Result<(), crate::TryReserveError>> = crate::map::iter::Values::default();
    let mut values_0_ref_0: &crate::map::iter::Values<isize, std::result::Result<(), crate::TryReserveError>> = &mut values_0;
    let mut intoiter_0: crate::map::iter::IntoIter<isize, isize> = crate::map::iter::IntoIter::default();
    let mut intoiter_0_ref_0: &crate::map::iter::IntoIter<isize, isize> = &mut intoiter_0;
    let mut intoiter_1: crate::map::iter::IntoIter<std::result::Result<(), crate::TryReserveError>, std::string::String> = crate::map::iter::IntoIter::default();
    let mut intoiter_1_ref_0: &crate::map::iter::IntoIter<std::result::Result<(), crate::TryReserveError>, std::string::String> = &mut intoiter_1;
    let mut usize_3: usize = 8461usize;
    let mut valuesmut_0: crate::map::iter::ValuesMut<isize, isize> = crate::map::iter::ValuesMut::default();
    let mut valuesmut_0_ref_0: &mut crate::map::iter::ValuesMut<isize, isize> = &mut valuesmut_0;
    let mut usize_4: usize = 5305usize;
    let mut itermut_0: crate::map::iter::IterMut<isize, isize> = crate::map::iter::IterMut::default();
    let mut itermut_0_ref_0: &mut crate::map::iter::IterMut<isize, isize> = &mut itermut_0;
    let mut usize_5: usize = 4611usize;
    let mut indexmapcore_2: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_5);
    let mut indexmapcore_2_ref_0: &crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_2;
    let mut iter_0: crate::set::iter::Iter<isize> = crate::set::iter::Iter::default();
    let mut isize_0: isize = -15044isize;
    let mut usize_6: usize = crate::set::iter::Iter::count(iter_0);
    let mut usize_7: usize = crate::map::core::IndexMapCore::capacity(indexmapcore_2_ref_0);
    let mut option_0: std::option::Option<(&isize, &mut isize)> = crate::map::iter::IterMut::next(itermut_0_ref_0);
    let mut tuple_0: (&isize, &mut isize) = std::option::Option::unwrap(option_0);
    let mut option_1: std::option::Option<&mut isize> = crate::map::iter::ValuesMut::nth(valuesmut_0_ref_0, usize_3);
    let mut isize_1: &mut isize = std::option::Option::unwrap(option_1);
    let mut tuple_1: (usize, std::option::Option<usize>) = crate::map::iter::IntoIter::size_hint(intoiter_0_ref_0);
    let mut indexmapcore_3: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::new();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_19() {
    rusty_monitor::set_test_id(19);
    let mut itermut_0: crate::map::iter::IterMut<isize, isize> = crate::map::iter::IterMut::default();
    let mut itermut_0_ref_0: &mut crate::map::iter::IterMut<isize, isize> = &mut itermut_0;
    let mut usize_0: usize = 859usize;
    let mut usize_1: usize = 7027usize;
    let mut usize_2: usize = 1608usize;
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_2);
    let mut indexmapcore_0_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_0;
    let mut usize_3: usize = 3252usize;
    let mut isize_0: isize = 11564isize;
    let mut isize_1: isize = -471isize;
    let mut intokeys_0: crate::map::iter::IntoKeys<isize, isize> = crate::map::iter::IntoKeys::default();
    let mut itermut_1: crate::map::iter::IterMut<isize, isize> = crate::map::iter::IterMut::default();
    let mut iter_0: crate::map::iter::Iter<isize, isize> = crate::map::iter::Iter::default();
    let mut iter_0_ref_0: &crate::map::iter::Iter<isize, isize> = &mut iter_0;
    let mut iter_1: crate::set::iter::Iter<isize> = crate::set::iter::Iter::default();
    let mut iter_1_ref_0: &crate::set::iter::Iter<isize> = &mut iter_1;
    let mut isize_2: isize = -10560isize;
    let mut indexmapcore_1: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_1_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_1;
    let mut refmut_0: crate::map::core::RefMut<isize, isize> = crate::map::core::IndexMapCore::borrow_mut(indexmapcore_1_ref_0);
    let mut usize_4: usize = 6627usize;
    let mut usize_5: usize = 1474usize;
    let mut getdisjointmuterror_0: GetDisjointMutError = crate::GetDisjointMutError::IndexOutOfBounds;
    let mut getdisjointmuterror_0_ref_0: &GetDisjointMutError = &mut getdisjointmuterror_0;
    let mut getdisjointmuterror_1: GetDisjointMutError = crate::GetDisjointMutError::clone(getdisjointmuterror_0_ref_0);
    let mut tuple_0: (usize, std::option::Option<usize>) = crate::set::iter::Iter::size_hint(iter_1_ref_0);
    let mut tuple_1: (usize, std::option::Option<usize>) = crate::map::iter::Iter::size_hint(iter_0_ref_0);
    let mut slice_0: &mut crate::map::slice::Slice<isize, isize> = crate::map::iter::IterMut::into_slice(itermut_1);
    let mut option_0: std::option::Option<isize> = crate::map::iter::IntoKeys::last(intokeys_0);
    crate::map::core::IndexMapCore::erase_indices(indexmapcore_0_ref_0, usize_1, usize_0);
    let mut getdisjointmuterror_1_ref_0: &GetDisjointMutError = &mut getdisjointmuterror_1;
    let mut option_1: std::option::Option<(&isize, &mut isize)> = crate::map::iter::IterMut::next(itermut_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_20() {
    rusty_monitor::set_test_id(20);
    let mut intokeys_0: crate::map::iter::IntoKeys<isize, isize> = crate::map::iter::IntoKeys::default();
    let mut intokeys_0_ref_0: &mut crate::map::iter::IntoKeys<isize, isize> = &mut intokeys_0;
    let mut valuesmut_0: crate::map::iter::ValuesMut<isize, isize> = crate::map::iter::ValuesMut::default();
    let mut usize_0: usize = 7901usize;
    let mut iter_0: crate::set::iter::Iter<isize> = crate::set::iter::Iter::default();
    let mut iter_0_ref_0: &mut crate::set::iter::Iter<isize> = &mut iter_0;
    let mut isize_0: isize = -5997isize;
    let mut isize_1: isize = 8359isize;
    let mut iter_1: crate::set::iter::Iter<isize> = crate::set::iter::Iter::default();
    let mut iter_1_ref_0: &crate::set::iter::Iter<isize> = &mut iter_1;
    let mut usize_1: usize = 7242usize;
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_1);
    let mut indexmapcore_0_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_0;
    let mut iter_2: crate::map::iter::Iter<isize, isize> = crate::map::iter::Iter::default();
    let mut iter_2_ref_0: &crate::map::iter::Iter<isize, isize> = &mut iter_2;
    let mut intovalues_0: crate::map::iter::IntoValues<isize, isize> = crate::map::iter::IntoValues::default();
    let mut intovalues_0_ref_0: &mut crate::map::iter::IntoValues<isize, isize> = &mut intovalues_0;
    let mut usize_2: usize = 4586usize;
    let mut usize_3: usize = 7112usize;
    let mut indexmapcore_1: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_3);
    let mut indexmapcore_1_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_1;
    let mut usize_4: usize = 8170usize;
    let mut result_0: std::result::Result<(), crate::TryReserveError> = crate::map::core::IndexMapCore::try_reserve_exact(indexmapcore_1_ref_0, usize_2);
    let mut option_0: std::option::Option<isize> = crate::map::iter::IntoValues::next(intovalues_0_ref_0);
    let mut tuple_0: (usize, std::option::Option<usize>) = crate::map::iter::Iter::size_hint(iter_2_ref_0);
    let mut refmut_0: crate::map::core::RefMut<isize, isize> = crate::map::core::IndexMapCore::borrow_mut(indexmapcore_0_ref_0);
    let mut tuple_1: (usize, std::option::Option<usize>) = crate::set::iter::Iter::size_hint(iter_1_ref_0);
    let mut option_1: std::option::Option<&isize> = crate::set::iter::Iter::nth(iter_0_ref_0, usize_0);
    let mut option_2: std::option::Option<&mut isize> = crate::map::iter::ValuesMut::last(valuesmut_0);
    let mut option_3: std::option::Option<isize> = crate::map::iter::IntoKeys::next(intokeys_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_21() {
    rusty_monitor::set_test_id(21);
    let mut iter_0: crate::set::iter::Iter<isize> = crate::set::iter::Iter::default();
    let mut iter_0_ref_0: &crate::set::iter::Iter<isize> = &mut iter_0;
    let mut intovalues_0: crate::map::iter::IntoValues<isize, isize> = crate::map::iter::IntoValues::default();
    let mut i32_0: i32 = 1908i32;
    let mut usize_0: usize = 5664usize;
    let mut iter_1: crate::set::iter::Iter<isize> = crate::set::iter::Iter::default();
    let mut iter_1_ref_0: &crate::set::iter::Iter<isize> = &mut iter_1;
    let mut usize_1: usize = 203usize;
    let mut keys_0: crate::map::iter::Keys<isize, isize> = crate::map::iter::Keys::default();
    let mut keys_0_ref_0: &crate::map::iter::Keys<isize, isize> = &mut keys_0;
    let mut indexmapcore_0: crate::map::core::IndexMapCore<usize, i32> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_0_ref_0: &crate::map::core::IndexMapCore<usize, i32> = &mut indexmapcore_0;
    let mut valuesmut_0: crate::map::iter::ValuesMut<isize, isize> = crate::map::iter::ValuesMut::default();
    let mut valuesmut_0_ref_0: &crate::map::iter::ValuesMut<isize, isize> = &mut valuesmut_0;
    let mut itermut2_0: crate::map::iter::IterMut2<isize, isize> = crate::map::iter::IterMut2::default();
    let mut usize_2: usize = 801usize;
    let mut isize_0: isize = -9107isize;
    let mut isize_1: isize = 19876isize;
    let mut isize_2: isize = 778isize;
    let mut isize_3: isize = 4681isize;
    let mut isize_4: isize = 15207isize;
    let mut usize_3: usize = 5135usize;
    let mut option_0: std::option::Option<(&mut isize, &mut isize)> = crate::map::iter::IterMut2::last(itermut2_0);
    let mut usize_4: usize = crate::map::iter::ValuesMut::len(valuesmut_0_ref_0);
    let mut tuple_0: (&mut isize, &mut isize) = std::option::Option::unwrap(option_0);
    let mut indexmapcore_1: crate::map::core::IndexMapCore<usize, i32> = crate::map::core::IndexMapCore::clone(indexmapcore_0_ref_0);
    let mut isize_5: &isize = crate::map::iter::Keys::index(keys_0_ref_0, usize_1);
    let mut tuple_1: (usize, std::option::Option<usize>) = crate::set::iter::Iter::size_hint(iter_1_ref_0);
    let mut indexmapcore_1_ref_0: &mut crate::map::core::IndexMapCore<usize, i32> = &mut indexmapcore_1;
    let mut usize_5: usize = crate::map::iter::IntoValues::count(intovalues_0);
    let mut iter_2: crate::set::iter::Iter<isize> = crate::set::iter::Iter::clone(iter_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_22() {
    rusty_monitor::set_test_id(22);
    let mut values_0: crate::map::iter::Values<isize, isize> = crate::map::iter::Values::default();
    let mut values_0_ref_0: &crate::map::iter::Values<isize, isize> = &mut values_0;
    let mut usize_0: usize = 2023usize;
    let mut isize_0: isize = -3919isize;
    let mut isize_1: isize = 2854isize;
    let mut indexmapcore_0: crate::map::core::IndexMapCore<u16, f32> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_0_ref_0: &crate::map::core::IndexMapCore<u16, f32> = &mut indexmapcore_0;
    let mut usize_1: usize = 2216usize;
    let mut isize_2: isize = 814isize;
    let mut isize_3: isize = 2255isize;
    let mut isize_4: isize = 14901isize;
    let mut isize_5: isize = 31763isize;
    let mut isize_6: isize = -19761isize;
    let mut isize_7: isize = 14052isize;
    let mut isize_8: isize = 2064isize;
    let mut usize_2: usize = 8839usize;
    let mut usize_3: usize = 9048usize;
    let mut intoiter_0: crate::set::iter::IntoIter<isize> = crate::set::iter::IntoIter::default();
    let mut intoiter_0_ref_0: &crate::set::iter::IntoIter<isize> = &mut intoiter_0;
    let mut valuesmut_0: crate::map::iter::ValuesMut<isize, isize> = crate::map::iter::ValuesMut::default();
    let mut values_1: crate::map::iter::Values<isize, std::result::Result<(), crate::TryReserveError>> = crate::map::iter::Values::default();
    let mut values_1_ref_0: &crate::map::iter::Values<isize, std::result::Result<(), crate::TryReserveError>> = &mut values_1;
    let mut intoiter_1: crate::map::iter::IntoIter<isize, isize> = crate::map::iter::IntoIter::default();
    let mut usize_4: usize = crate::map::iter::IntoIter::count(intoiter_1);
    let mut option_0: std::option::Option<&mut isize> = crate::map::iter::ValuesMut::last(valuesmut_0);
    let mut usize_5: usize = crate::set::iter::IntoIter::len(intoiter_0_ref_0);
    let mut isize_9: &mut isize = std::option::Option::unwrap(option_0);
    let mut indexmapcore_1: crate::map::core::IndexMapCore<u16, f32> = crate::map::core::IndexMapCore::clone(indexmapcore_0_ref_0);
    let mut indexmapcore_1_ref_0: &crate::map::core::IndexMapCore<u16, f32> = &mut indexmapcore_1;
    let mut usize_6: usize = crate::map::core::IndexMapCore::len(indexmapcore_1_ref_0);
    let mut usize_7: usize = crate::map::iter::Values::len(values_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_23() {
    rusty_monitor::set_test_id(23);
    let mut isize_0: isize = 11554isize;
    let mut isize_1: isize = 18639isize;
    let mut isize_2: isize = -27623isize;
    let mut isize_3: isize = 9504isize;
    let mut u64_0: u64 = 154u64;
    let mut intovalues_0: crate::map::iter::IntoValues<isize, isize> = crate::map::iter::IntoValues::default();
    let mut intovalues_0_ref_0: &crate::map::iter::IntoValues<isize, isize> = &mut intovalues_0;
    let mut valuesmut_0: crate::map::iter::ValuesMut<isize, isize> = crate::map::iter::ValuesMut::default();
    let mut valuesmut_0_ref_0: &crate::map::iter::ValuesMut<isize, isize> = &mut valuesmut_0;
    let mut usize_0: usize = 3049usize;
    let mut usize_1: usize = 7442usize;
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_1);
    let mut indexmapcore_0_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_0;
    let mut usize_2: usize = 9453usize;
    let mut intokeys_0: crate::map::iter::IntoKeys<isize, isize> = crate::map::iter::IntoKeys::default();
    let mut intokeys_0_ref_0: &crate::map::iter::IntoKeys<isize, isize> = &mut intokeys_0;
    let mut usize_3: usize = 9166usize;
    let mut usize_4: usize = 6538usize;
    let mut iter_0: crate::set::iter::Iter<isize> = crate::set::iter::Iter::default();
    let mut iter_0_ref_0: &crate::set::iter::Iter<isize> = &mut iter_0;
    let mut intokeys_1: crate::map::iter::IntoKeys<isize, isize> = crate::map::iter::IntoKeys::default();
    let mut intoiter_0: crate::map::iter::IntoIter<isize, isize> = crate::map::iter::IntoIter::default();
    let mut intoiter_0_ref_0: &mut crate::map::iter::IntoIter<isize, isize> = &mut intoiter_0;
    let mut option_0: std::option::Option<(isize, isize)> = crate::map::iter::IntoIter::next(intoiter_0_ref_0);
    let mut usize_5: usize = crate::map::iter::IntoKeys::count(intokeys_1);
    let mut tuple_0: (isize, isize) = std::option::Option::unwrap(option_0);
    let mut usize_6: usize = crate::set::iter::Iter::len(iter_0_ref_0);
    let mut usize_7: usize = crate::map::iter::IntoKeys::len(intokeys_0_ref_0);
    crate::map::core::IndexMapCore::reserve(indexmapcore_0_ref_0, usize_0);
    let mut tuple_1: (usize, std::option::Option<usize>) = crate::map::iter::ValuesMut::size_hint(valuesmut_0_ref_0);
    let mut tuple_2: (usize, std::option::Option<usize>) = crate::map::iter::IntoValues::size_hint(intovalues_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_24() {
    rusty_monitor::set_test_id(24);
    let mut usize_0: usize = 4313usize;
    let mut intoiter_0: crate::set::iter::IntoIter<isize> = crate::set::iter::IntoIter::default();
    let mut intoiter_0_ref_0: &mut crate::set::iter::IntoIter<isize> = &mut intoiter_0;
    let mut usize_1: usize = 5982usize;
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_1);
    let mut indexmapcore_0_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_0;
    let mut indexmapcore_1: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_1_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_1;
    let mut indexmapcore_2: crate::map::core::IndexMapCore<std::result::Result<(), crate::TryReserveError>, std::result::Result<(), crate::TryReserveError>> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_2_ref_0: &crate::map::core::IndexMapCore<std::result::Result<(), crate::TryReserveError>, std::result::Result<(), crate::TryReserveError>> = &mut indexmapcore_2;
    let mut intoiter_1: crate::set::iter::IntoIter<isize> = crate::set::iter::IntoIter::default();
    let mut usize_2: usize = 6111usize;
    let mut isize_0: isize = -24780isize;
    let mut isize_1: isize = 4062isize;
    let mut indexmapcore_3: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_3_ref_0: &crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_3;
    let mut isize_2: isize = 3620isize;
    let mut isize_3: isize = 10917isize;
    let mut isize_4: isize = 8502isize;
    let mut isize_4_ref_0: &isize = &mut isize_4;
    let mut itermut_0: crate::map::iter::IterMut<isize, isize> = crate::map::iter::IterMut::default();
    let mut itermut_0_ref_0: &mut crate::map::iter::IterMut<isize, isize> = &mut itermut_0;
    let mut itermut2_0: crate::map::iter::IterMut2<isize, isize> = crate::map::iter::IterMut2::default();
    let mut slice_0: &mut crate::map::slice::Slice<isize, isize> = crate::map::iter::IterMut2::into_slice(itermut2_0);
    let mut option_0: std::option::Option<(&isize, &mut isize)> = crate::map::iter::IterMut::next(itermut_0_ref_0);
    let mut tuple_0: (&isize, &mut isize) = std::option::Option::unwrap(option_0);
    let mut usize_3: usize = crate::map::core::IndexMapCore::capacity(indexmapcore_3_ref_0);
    let mut usize_4: usize = crate::set::iter::IntoIter::count(intoiter_1);
    crate::map::core::IndexMapCore::append_unchecked(indexmapcore_1_ref_0, indexmapcore_0_ref_0);
    let mut iter_0: crate::map::iter::Iter<isize, isize> = crate::map::iter::Iter::default();
    let mut intovalues_0: crate::map::iter::IntoValues<isize, isize> = crate::map::iter::IntoValues::default();
    let mut option_1: std::option::Option<isize> = crate::set::iter::IntoIter::nth(intoiter_0_ref_0, usize_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_25() {
    rusty_monitor::set_test_id(25);
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_0_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_0;
    let mut isize_0: isize = 1574isize;
    let mut isize_1: isize = -1192isize;
    let mut isize_2: isize = 8717isize;
    let mut intoiter_0: crate::set::iter::IntoIter<isize> = crate::set::iter::IntoIter::default();
    let mut intoiter_0_ref_0: &mut crate::set::iter::IntoIter<isize> = &mut intoiter_0;
    let mut itermut2_0: crate::map::iter::IterMut2<isize, isize> = crate::map::iter::IterMut2::default();
    let mut itermut2_1: crate::map::iter::IterMut2<std::string::String, std::string::String> = crate::map::iter::IterMut2::default();
    let mut itermut2_1_ref_0: &crate::map::iter::IterMut2<std::string::String, std::string::String> = &mut itermut2_1;
    let mut indexmapcore_1: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_1_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_1;
    let mut usize_0: usize = 6652usize;
    let mut keys_0: crate::map::iter::Keys<isize, isize> = crate::map::iter::Keys::default();
    let mut keys_0_ref_0: &crate::map::iter::Keys<isize, isize> = &mut keys_0;
    let mut iter_0: crate::set::iter::Iter<std::result::Result<(), crate::TryReserveError>> = crate::set::iter::Iter::default();
    let mut iter_0_ref_0: &crate::set::iter::Iter<std::result::Result<(), crate::TryReserveError>> = &mut iter_0;
    let mut iter_1: crate::map::iter::Iter<isize, isize> = crate::map::iter::Iter::default();
    let mut iter_1_ref_0: &crate::map::iter::Iter<isize, isize> = &mut iter_1;
    let mut iter_2: crate::map::iter::Iter<isize, isize> = crate::map::iter::Iter::clone(iter_1_ref_0);
    let mut keys_1: crate::map::iter::Keys<isize, isize> = crate::map::iter::Keys::clone(keys_0_ref_0);
    crate::map::core::IndexMapCore::clear(indexmapcore_1_ref_0);
    let mut keys_1_ref_0: &crate::map::iter::Keys<isize, isize> = &mut keys_1;
    let mut usize_1: usize = crate::map::iter::Keys::len(keys_1_ref_0);
    let mut option_0: std::option::Option<(&isize, &isize)> = crate::map::iter::Iter::last(iter_2);
    let mut slice_0: &mut crate::map::slice::Slice<isize, isize> = crate::map::iter::IterMut2::into_slice(itermut2_0);
    let mut option_1: std::option::Option<isize> = crate::set::iter::IntoIter::next(intoiter_0_ref_0);
    let mut isize_3: isize = std::option::Option::unwrap(option_1);
    let mut tuple_0: (&isize, &isize) = std::option::Option::unwrap(option_0);
    let mut option_2: std::option::Option<(isize, isize)> = crate::map::core::IndexMapCore::pop(indexmapcore_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_27() {
    rusty_monitor::set_test_id(27);
    let mut getdisjointmuterror_0: GetDisjointMutError = crate::GetDisjointMutError::IndexOutOfBounds;
    let mut getdisjointmuterror_0_ref_0: &GetDisjointMutError = &mut getdisjointmuterror_0;
    let mut getdisjointmuterror_1: GetDisjointMutError = crate::GetDisjointMutError::IndexOutOfBounds;
    let mut getdisjointmuterror_1_ref_0: &GetDisjointMutError = &mut getdisjointmuterror_1;
    let mut itermut_0: crate::map::iter::IterMut<isize, isize> = crate::map::iter::IterMut::default();
    let mut itermut_0_ref_0: &mut crate::map::iter::IterMut<isize, isize> = &mut itermut_0;
    let mut keys_0: crate::map::iter::Keys<isize, isize> = crate::map::iter::Keys::default();
    let mut isize_0: isize = -4175isize;
    let mut isize_0_ref_0: &isize = &mut isize_0;
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_0_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_0;
    let mut refmut_0: crate::map::core::RefMut<isize, isize> = crate::map::core::IndexMapCore::borrow_mut(indexmapcore_0_ref_0);
    let mut valuesmut_0: crate::map::iter::ValuesMut<isize, isize> = crate::map::iter::ValuesMut::default();
    let mut valuesmut_0_ref_0: &crate::map::iter::ValuesMut<isize, isize> = &mut valuesmut_0;
    let mut itermut2_0: crate::map::iter::IterMut2<isize, isize> = crate::map::iter::IterMut2::default();
    let mut isize_1: isize = 4546isize;
    let mut usize_0: usize = 6847usize;
    let mut isize_2: isize = -4055isize;
    let mut option_0: std::option::Option<(&mut isize, &mut isize)> = crate::map::iter::IterMut2::last(itermut2_0);
    let mut usize_1: usize = crate::map::iter::ValuesMut::len(valuesmut_0_ref_0);
    let mut iter_0: crate::set::iter::Iter<isize> = crate::set::iter::Iter::default();
    let mut option_1: std::option::Option<&isize> = crate::map::iter::Keys::last(keys_0);
    let mut option_2: std::option::Option<(&isize, &mut isize)> = crate::map::iter::IterMut::next(itermut_0_ref_0);
    let mut iter_0_ref_0: &crate::set::iter::Iter<isize> = &mut iter_0;
    let mut iter_1: crate::set::iter::Iter<isize> = crate::set::iter::Iter::clone(iter_0_ref_0);
    let mut iter_1_ref_0: &crate::set::iter::Iter<isize> = &mut iter_1;
    let mut iter_2: crate::set::iter::Iter<isize> = crate::set::iter::Iter::clone(iter_1_ref_0);
    let mut option_3: std::option::Option<&isize> = crate::set::iter::Iter::last(iter_2);
    let mut bool_0: bool = crate::GetDisjointMutError::eq(getdisjointmuterror_1_ref_0, getdisjointmuterror_0_ref_0);
    let mut keys_1: crate::map::iter::Keys<isize, isize> = crate::map::iter::Keys::default();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_29() {
    rusty_monitor::set_test_id(29);
    let mut usize_0: usize = 323usize;
    let mut usize_1: usize = 8220usize;
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_1);
    let mut indexmapcore_0_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_0;
    let mut usize_2: usize = 8161usize;
    let mut iter_0: crate::set::iter::Iter<isize> = crate::set::iter::Iter::default();
    let mut iter_0_ref_0: &mut crate::set::iter::Iter<isize> = &mut iter_0;
    let mut isize_0: isize = 12160isize;
    let mut isize_1: isize = -2461isize;
    let mut usize_3: usize = 9164usize;
    let mut indexmapcore_1: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_3);
    let mut indexmapcore_1_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_1;
    let mut keys_0: crate::map::iter::Keys<isize, isize> = crate::map::iter::Keys::default();
    let mut usize_4: usize = 995usize;
    let mut indexmapcore_2: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_2_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_2;
    let mut isize_2: isize = -8040isize;
    let mut isize_2_ref_0: &isize = &mut isize_2;
    let mut usize_5: usize = 2772usize;
    let mut usize_6: usize = 26usize;
    let mut isize_3: isize = 9210isize;
    let mut isize_4: isize = 6104isize;
    let mut isize_5: isize = 13043isize;
    let mut isize_6: isize = 7181isize;
    let mut isize_7: isize = 3011isize;
    let mut usize_7: usize = 6189usize;
    let mut result_0: std::result::Result<(), crate::TryReserveError> = crate::map::core::IndexMapCore::try_reserve(indexmapcore_2_ref_0, usize_4);
    let mut tuple_0: () = std::result::Result::unwrap(result_0);
    let mut option_0: std::option::Option<&isize> = crate::map::iter::Keys::last(keys_0);
    let mut option_1: std::option::Option<&isize> = crate::set::iter::Iter::nth(iter_0_ref_0, usize_2);
    crate::map::core::IndexMapCore::reserve(indexmapcore_0_ref_0, usize_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_30() {
    rusty_monitor::set_test_id(30);
    let mut isize_0: isize = 33424isize;
    let mut isize_1: isize = -6229isize;
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_0_ref_0: &crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_0;
    let mut usize_0: usize = 4664usize;
    let mut usize_1: usize = 2462usize;
    let mut indexmapcore_1: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_1);
    let mut indexmapcore_1_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_1;
    let mut iter_0: crate::map::iter::Iter<isize, isize> = crate::map::iter::Iter::default();
    let mut iter_0_ref_0: &crate::map::iter::Iter<isize, isize> = &mut iter_0;
    let mut isize_2: isize = -6135isize;
    let mut isize_3: isize = 16116isize;
    let mut valuesmut_0: crate::map::iter::ValuesMut<isize, isize> = crate::map::iter::ValuesMut::default();
    let mut valuesmut_0_ref_0: &crate::map::iter::ValuesMut<isize, isize> = &mut valuesmut_0;
    let mut isize_4: isize = 10727isize;
    let mut usize_2: usize = 7343usize;
    let mut indexmapcore_2: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_2);
    let mut indexmapcore_2_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_2;
    let mut refmut_0: crate::map::core::RefMut<isize, isize> = crate::map::core::IndexMapCore::borrow_mut(indexmapcore_2_ref_0);
    let mut usize_3: usize = 1531usize;
    let mut isize_5: isize = -5544isize;
    let mut isize_6: isize = -58isize;
    let mut isize_7: isize = 19578isize;
    let mut isize_8: isize = 11493isize;
    let mut isize_9: isize = 407isize;
    let mut usize_4: usize = 4774usize;
    let mut iter_1: crate::set::iter::Iter<isize> = crate::set::iter::Iter::default();
    let mut tuple_0: (usize, std::option::Option<usize>) = crate::map::iter::ValuesMut::size_hint(valuesmut_0_ref_0);
    let mut iter_2: crate::map::iter::Iter<isize, isize> = crate::map::iter::Iter::clone(iter_0_ref_0);
    let mut option_0: std::option::Option<(isize, isize)> = crate::map::core::IndexMapCore::shift_remove_index(indexmapcore_1_ref_0, usize_0);
    let mut bucket_slice_0: &[crate::Bucket<isize, isize>] = crate::map::core::IndexMapCore::as_entries(indexmapcore_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_31() {
    rusty_monitor::set_test_id(31);
    let mut usize_0: usize = 9421usize;
    let mut iter_0: crate::map::iter::Iter<isize, isize> = crate::map::iter::Iter::default();
    let mut iter_0_ref_0: &mut crate::map::iter::Iter<isize, isize> = &mut iter_0;
    let mut valuesmut_0: crate::map::iter::ValuesMut<isize, std::result::Result<(), crate::TryReserveError>> = crate::map::iter::ValuesMut::default();
    let mut valuesmut_0_ref_0: &crate::map::iter::ValuesMut<isize, std::result::Result<(), crate::TryReserveError>> = &mut valuesmut_0;
    let mut keys_0: crate::map::iter::Keys<isize, isize> = crate::map::iter::Keys::default();
    let mut keys_0_ref_0: &crate::map::iter::Keys<isize, isize> = &mut keys_0;
    let mut isize_0: isize = 9332isize;
    let mut isize_1: isize = 5964isize;
    let mut keys_1: crate::map::iter::Keys<std::result::Result<(), crate::TryReserveError>, isize> = crate::map::iter::Keys::default();
    let mut keys_1_ref_0: &crate::map::iter::Keys<std::result::Result<(), crate::TryReserveError>, isize> = &mut keys_1;
    let mut isize_2: isize = -2779isize;
    let mut isize_3: isize = 17156isize;
    let mut isize_4: isize = -10697isize;
    let mut isize_5: isize = 558isize;
    let mut isize_6: isize = 12345isize;
    let mut isize_7: isize = 25161isize;
    let mut usize_1: usize = 6956usize;
    let mut keys_2: crate::map::iter::Keys<isize, isize> = crate::map::iter::Keys::default();
    let mut keys_2_ref_0: &crate::map::iter::Keys<isize, isize> = &mut keys_2;
    let mut usize_2: usize = 2758usize;
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_2);
    let mut indexmapcore_0_ref_0: &crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_0;
    let mut isize_8: isize = -325isize;
    let mut usize_3: usize = 4671usize;
    let mut indexmapcore_1: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_1_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_1;
    let mut usize_4: usize = 8416usize;
    let mut result_0: std::result::Result<(), crate::TryReserveError> = crate::map::core::IndexMapCore::try_reserve(indexmapcore_1_ref_0, usize_3);
    let mut usize_5: usize = crate::map::core::IndexMapCore::capacity(indexmapcore_0_ref_0);
    let mut isize_9: &isize = crate::map::iter::Keys::index(keys_2_ref_0, usize_1);
    let mut keys_3: crate::map::iter::Keys<isize, isize> = crate::map::iter::Keys::clone(keys_0_ref_0);
    let mut option_0: std::option::Option<(&isize, &isize)> = crate::map::iter::Iter::nth(iter_0_ref_0, usize_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_32() {
    rusty_monitor::set_test_id(32);
    let mut isize_0: isize = -3934isize;
    let mut isize_1: isize = -828isize;
    let mut isize_2: isize = 8273isize;
    let mut isize_3: isize = -897isize;
    let mut intokeys_0: crate::map::iter::IntoKeys<isize, isize> = crate::map::iter::IntoKeys::default();
    let mut valuesmut_0: crate::map::iter::ValuesMut<isize, isize> = crate::map::iter::ValuesMut::default();
    let mut valuesmut_0_ref_0: &crate::map::iter::ValuesMut<isize, isize> = &mut valuesmut_0;
    let mut isize_4: isize = 4563isize;
    let mut isize_5: isize = 3353isize;
    let mut usize_0: usize = 9953usize;
    let mut isize_6: isize = 6623isize;
    let mut isize_7: isize = -10415isize;
    let mut isize_8: isize = 12361isize;
    let mut usize_1: usize = 8333usize;
    let mut usize_2: usize = 8302usize;
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_2);
    let mut indexmapcore_0_ref_0: &crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_0;
    let mut usize_3: usize = 7035usize;
    let mut isize_9: isize = -12491isize;
    let mut isize_10: isize = -20585isize;
    let mut isize_11: isize = 7524isize;
    let mut isize_12: isize = -10146isize;
    let mut isize_13: isize = -7379isize;
    let mut usize_4: usize = 8761usize;
    let mut usize_5: usize = 4700usize;
    let mut isize_14: isize = 13393isize;
    let mut usize_6: usize = crate::map::core::IndexMapCore::capacity(indexmapcore_0_ref_0);
    let mut intoiter_0: crate::set::iter::IntoIter<isize> = crate::set::iter::IntoIter::default();
    let mut tuple_0: (usize, std::option::Option<usize>) = crate::map::iter::ValuesMut::size_hint(valuesmut_0_ref_0);
    let mut intoiter_1: crate::map::iter::IntoIter<isize, isize> = crate::map::iter::IntoIter::default();
    let mut option_0: std::option::Option<isize> = crate::map::iter::IntoKeys::last(intokeys_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_34() {
    rusty_monitor::set_test_id(34);
    let mut usize_0: usize = 3842usize;
    let mut usize_1: usize = 7759usize;
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_1);
    let mut indexmapcore_0_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_0;
    let mut result_0: std::result::Result<(), crate::TryReserveError> = crate::map::core::IndexMapCore::try_reserve(indexmapcore_0_ref_0, usize_0);
    let mut usize_2: usize = 9979usize;
    let mut indexmapcore_1: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_1_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_1;
    let mut result_1: std::result::Result<(), crate::TryReserveError> = crate::map::core::IndexMapCore::try_reserve_exact(indexmapcore_1_ref_0, usize_2);
    let mut usize_3: usize = 7629usize;
    let mut valuesmut_0: crate::map::iter::ValuesMut<isize, isize> = crate::map::iter::ValuesMut::default();
    let mut valuesmut_0_ref_0: &mut crate::map::iter::ValuesMut<isize, isize> = &mut valuesmut_0;
    let mut intoiter_0: crate::set::iter::IntoIter<isize> = crate::set::iter::IntoIter::default();
    let mut intoiter_0_ref_0: &crate::set::iter::IntoIter<isize> = &mut intoiter_0;
    let mut keys_0: crate::map::iter::Keys<isize, isize> = crate::map::iter::Keys::default();
    let mut keys_0_ref_0: &crate::map::iter::Keys<isize, isize> = &mut keys_0;
    let mut valuesmut_1: crate::map::iter::ValuesMut<isize, std::string::String> = crate::map::iter::ValuesMut::default();
    let mut valuesmut_1_ref_0: &crate::map::iter::ValuesMut<isize, std::string::String> = &mut valuesmut_1;
    let mut getdisjointmuterror_0: GetDisjointMutError = crate::GetDisjointMutError::OverlappingIndices;
    let mut getdisjointmuterror_0_ref_0: &GetDisjointMutError = &mut getdisjointmuterror_0;
    let mut isize_0: isize = 12493isize;
    let mut isize_1: isize = 3253isize;
    let mut usize_4: usize = 4997usize;
    let mut indexmapcore_2: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_4);
    let mut indexmapcore_2_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_2;
    let mut isize_2: isize = -1796isize;
    let mut isize_3: isize = 12481isize;
    let mut values_0: crate::map::iter::Values<isize, isize> = crate::map::iter::Values::default();
    let mut values_0_ref_0: &mut crate::map::iter::Values<isize, isize> = &mut values_0;
    let mut option_0: std::option::Option<&isize> = crate::map::iter::Values::next(values_0_ref_0);
    let mut getdisjointmuterror_1: GetDisjointMutError = crate::GetDisjointMutError::IndexOutOfBounds;
    let mut indexmapcore_3: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::new();
    let mut usize_5: usize = crate::map::iter::Keys::len(keys_0_ref_0);
    let mut tuple_0: (usize, std::option::Option<usize>) = crate::set::iter::IntoIter::size_hint(intoiter_0_ref_0);
    let mut option_1: std::option::Option<&mut isize> = crate::map::iter::ValuesMut::next(valuesmut_0_ref_0);
    let mut intoiter_1: crate::set::iter::IntoIter<isize> = crate::set::iter::IntoIter::default();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_35() {
    rusty_monitor::set_test_id(35);
    let mut intoiter_0: crate::set::iter::IntoIter<i128> = crate::set::iter::IntoIter::default();
    let mut intoiter_0_ref_0: &crate::set::iter::IntoIter<i128> = &mut intoiter_0;
    let mut itermut2_0: crate::map::iter::IterMut2<isize, isize> = crate::map::iter::IterMut2::default();
    let mut usize_0: usize = 3763usize;
    let mut iter_0: crate::map::iter::Iter<std::string::String, std::string::String> = crate::map::iter::Iter::default();
    let mut iter_0_ref_0: &crate::map::iter::Iter<std::string::String, std::string::String> = &mut iter_0;
    let mut intoiter_1: crate::map::iter::IntoIter<isize, isize> = crate::map::iter::IntoIter::default();
    let mut iter_1: crate::map::iter::Iter<isize, isize> = crate::map::iter::Iter::default();
    let mut iter_1_ref_0: &crate::map::iter::Iter<isize, isize> = &mut iter_1;
    let mut usize_1: usize = 1773usize;
    let mut usize_2: usize = 2336usize;
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_2);
    let mut indexmapcore_0_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_0;
    let mut isize_0: isize = -5731isize;
    let mut isize_1: isize = 18614isize;
    let mut isize_2: isize = -13685isize;
    let mut iter_2: crate::set::iter::Iter<isize> = crate::set::iter::Iter::default();
    let mut iter_2_ref_0: &crate::set::iter::Iter<isize> = &mut iter_2;
    let mut usize_3: usize = 7016usize;
    let mut indexmapcore_1: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_3);
    let mut indexmapcore_1_ref_0: &crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_1;
    let mut usize_4: usize = crate::map::core::IndexMapCore::capacity(indexmapcore_1_ref_0);
    let mut usize_5: usize = crate::set::iter::Iter::len(iter_2_ref_0);
    crate::map::core::IndexMapCore::reserve(indexmapcore_0_ref_0, usize_1);
    let mut iter_3: crate::map::iter::Iter<isize, isize> = crate::map::iter::Iter::clone(iter_1_ref_0);
    let mut usize_6: usize = crate::map::iter::IntoIter::count(intoiter_1);
    let mut iter_4: crate::set::iter::Iter<isize> = crate::set::iter::Iter::default();
    let mut iter_3_ref_0: &crate::map::iter::Iter<isize, isize> = &mut iter_3;
    let mut slice_0: &crate::map::slice::Slice<isize, isize> = crate::map::iter::Iter::as_slice(iter_3_ref_0);
    let mut slice_1: &mut crate::map::slice::Slice<isize, isize> = crate::map::iter::IterMut2::into_slice(itermut2_0);
    let mut intoiter_2: crate::set::iter::IntoIter<i128> = crate::set::iter::IntoIter::clone(intoiter_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_36() {
    rusty_monitor::set_test_id(36);
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_0_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_0;
    let mut usize_0: usize = 2943usize;
    let mut intoiter_0: crate::map::iter::IntoIter<std::result::Result<(), crate::TryReserveError>, std::result::Result<(), crate::TryReserveError>> = crate::map::iter::IntoIter::default();
    let mut intoiter_0_ref_0: &crate::map::iter::IntoIter<std::result::Result<(), crate::TryReserveError>, std::result::Result<(), crate::TryReserveError>> = &mut intoiter_0;
    let mut usize_1: usize = 8752usize;
    let mut valuesmut_0: crate::map::iter::ValuesMut<isize, isize> = crate::map::iter::ValuesMut::default();
    let mut valuesmut_0_ref_0: &mut crate::map::iter::ValuesMut<isize, isize> = &mut valuesmut_0;
    let mut usize_2: usize = 4871usize;
    let mut usize_3: usize = 2012usize;
    let mut isize_0: isize = -1890isize;
    let mut isize_1: isize = 4813isize;
    let mut isize_2: isize = 1544isize;
    let mut isize_3: isize = 7452isize;
    let mut isize_4: isize = -2302isize;
    let mut usize_4: usize = 9839usize;
    let mut usize_5: usize = 7101usize;
    let mut usize_6: usize = 3414usize;
    let mut itermut_0: crate::map::iter::IterMut<std::result::Result<(), crate::TryReserveError>, std::string::String> = crate::map::iter::IterMut::default();
    let mut itermut_0_ref_0: &crate::map::iter::IterMut<std::result::Result<(), crate::TryReserveError>, std::string::String> = &mut itermut_0;
    let mut valuesmut_1: crate::map::iter::ValuesMut<isize, isize> = crate::map::iter::ValuesMut::default();
    let mut valuesmut_1_ref_0: &mut crate::map::iter::ValuesMut<isize, isize> = &mut valuesmut_1;
    let mut isize_5: isize = 14368isize;
    let mut option_0: std::option::Option<&mut isize> = crate::map::iter::ValuesMut::next(valuesmut_1_ref_0);
    let mut isize_6: &mut isize = std::option::Option::unwrap(option_0);
    let mut option_1: std::option::Option<&mut isize> = crate::map::iter::ValuesMut::nth(valuesmut_0_ref_0, usize_1);
    let mut valuesmut_2: crate::map::iter::ValuesMut<isize, isize> = crate::map::iter::ValuesMut::default();
    let mut valuesmut_2_ref_0: &crate::map::iter::ValuesMut<isize, isize> = &mut valuesmut_2;
    let mut tuple_0: (usize, std::option::Option<usize>) = crate::map::iter::ValuesMut::size_hint(valuesmut_2_ref_0);
    let mut bucket_slice_0: &mut [crate::Bucket<isize, isize>] = crate::map::core::IndexMapCore::as_entries_mut(indexmapcore_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_37() {
    rusty_monitor::set_test_id(37);
    let mut keys_0: crate::map::iter::Keys<isize, isize> = crate::map::iter::Keys::default();
    let mut isize_0: isize = 7185isize;
    let mut isize_1: isize = -7472isize;
    let mut isize_2: isize = -403isize;
    let mut usize_0: usize = 5642usize;
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_0_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_0;
    let mut isize_3: isize = -11332isize;
    let mut isize_4: isize = -93isize;
    let mut isize_5: isize = 7372isize;
    let mut isize_6: isize = -1089isize;
    let mut isize_7: isize = -2730isize;
    let mut usize_1: usize = 1959usize;
    let mut usize_2: usize = 1021usize;
    let mut usize_3: usize = 8089usize;
    let mut iter_0: crate::set::iter::Iter<isize> = crate::set::iter::Iter::default();
    let mut iter_0_ref_0: &mut crate::set::iter::Iter<isize> = &mut iter_0;
    let mut itermut2_0: crate::map::iter::IterMut2<isize, isize> = crate::map::iter::IterMut2::default();
    let mut itermut2_0_ref_0: &crate::map::iter::IterMut2<isize, isize> = &mut itermut2_0;
    let mut intokeys_0: crate::map::iter::IntoKeys<std::result::Result<(), crate::TryReserveError>, isize> = crate::map::iter::IntoKeys::default();
    let mut intokeys_0_ref_0: &crate::map::iter::IntoKeys<std::result::Result<(), crate::TryReserveError>, isize> = &mut intokeys_0;
    let mut itermut_0: crate::map::iter::IterMut<isize, isize> = crate::map::iter::IterMut::default();
    let mut itermut_0_ref_0: &mut crate::map::iter::IterMut<isize, isize> = &mut itermut_0;
    let mut option_0: std::option::Option<(&isize, &mut isize)> = crate::map::iter::IterMut::next(itermut_0_ref_0);
    let mut tuple_0: (&isize, &mut isize) = std::option::Option::unwrap(option_0);
    let mut tuple_1: (usize, std::option::Option<usize>) = crate::map::iter::IterMut2::size_hint(itermut2_0_ref_0);
    let mut option_1: std::option::Option<&isize> = crate::set::iter::Iter::nth(iter_0_ref_0, usize_3);
    crate::map::core::IndexMapCore::reserve_exact(indexmapcore_0_ref_0, usize_0);
    let mut option_2: std::option::Option<&isize> = crate::map::iter::Keys::last(keys_0);
    let mut isize_8: &isize = std::option::Option::unwrap(option_2);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_41() {
    rusty_monitor::set_test_id(41);
    let mut intokeys_0: crate::map::iter::IntoKeys<std::string::String, isize> = crate::map::iter::IntoKeys::default();
    let mut intokeys_0_ref_0: &crate::map::iter::IntoKeys<std::string::String, isize> = &mut intokeys_0;
    let mut iter_0: crate::map::iter::Iter<isize, isize> = crate::map::iter::Iter::default();
    let mut iter_0_ref_0: &mut crate::map::iter::Iter<isize, isize> = &mut iter_0;
    let mut isize_0: isize = -6703isize;
    let mut isize_1: isize = -12375isize;
    let mut isize_2: isize = -10768isize;
    let mut isize_3: isize = -313isize;
    let mut isize_4: isize = 5112isize;
    let mut intokeys_1: crate::map::iter::IntoKeys<isize, isize> = crate::map::iter::IntoKeys::default();
    let mut intokeys_1_ref_0: &mut crate::map::iter::IntoKeys<isize, isize> = &mut intokeys_1;
    let mut itermut_0: crate::map::iter::IterMut<isize, isize> = crate::map::iter::IterMut::default();
    let mut itermut_0_ref_0: &crate::map::iter::IterMut<isize, isize> = &mut itermut_0;
    let mut isize_5: isize = 5835isize;
    let mut isize_6: isize = -1694isize;
    let mut intovalues_0: crate::map::iter::IntoValues<isize, isize> = crate::map::iter::IntoValues::default();
    let mut intokeys_2: crate::map::iter::IntoKeys<isize, isize> = crate::map::iter::IntoKeys::default();
    let mut usize_0: usize = 683usize;
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_0);
    let mut indexmapcore_0_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_0;
    let mut usize_1: usize = 7815usize;
    crate::map::core::IndexMapCore::rebuild_hash_table(indexmapcore_0_ref_0);
    let mut usize_2: usize = crate::map::iter::IntoKeys::count(intokeys_2);
    let mut usize_3: usize = crate::map::iter::IntoValues::count(intovalues_0);
    let mut slice_0: &crate::map::slice::Slice<isize, isize> = crate::map::iter::IterMut::as_slice(itermut_0_ref_0);
    let mut option_0: std::option::Option<isize> = crate::map::iter::IntoKeys::next(intokeys_1_ref_0);
    let mut isize_7: isize = std::option::Option::unwrap(option_0);
    let mut valuesmut_0: crate::map::iter::ValuesMut<isize, isize> = crate::map::iter::ValuesMut::default();
    let mut usize_4: usize = crate::map::iter::ValuesMut::count(valuesmut_0);
    let mut option_1: std::option::Option<(&isize, &isize)> = crate::map::iter::Iter::next(iter_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_45() {
    rusty_monitor::set_test_id(45);
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_0_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_0;
    let mut keys_0: crate::map::iter::Keys<isize, isize> = crate::map::iter::Keys::default();
    let mut keys_0_ref_0: &mut crate::map::iter::Keys<isize, isize> = &mut keys_0;
    let mut usize_0: usize = 9552usize;
    let mut intoiter_0: crate::set::iter::IntoIter<std::string::String> = crate::set::iter::IntoIter::default();
    let mut intoiter_0_ref_0: &crate::set::iter::IntoIter<std::string::String> = &mut intoiter_0;
    let mut iter_0: crate::map::iter::Iter<isize, isize> = crate::map::iter::Iter::default();
    let mut iter_0_ref_0: &crate::map::iter::Iter<isize, isize> = &mut iter_0;
    let mut usize_1: usize = 1643usize;
    let mut indexmapcore_1: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_1_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_1;
    let mut intokeys_0: crate::map::iter::IntoKeys<isize, isize> = crate::map::iter::IntoKeys::default();
    let mut intokeys_0_ref_0: &crate::map::iter::IntoKeys<isize, isize> = &mut intokeys_0;
    let mut indexmapcore_2: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_2_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_2;
    let mut usize_2: usize = 7023usize;
    let mut indexmapcore_3: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_2);
    let mut indexmapcore_3_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_3;
    let mut usize_3: usize = 4642usize;
    let mut indexmapcore_4: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_4_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_4;
    crate::map::core::IndexMapCore::reserve_exact(indexmapcore_4_ref_0, usize_3);
    crate::map::core::IndexMapCore::append_unchecked(indexmapcore_3_ref_0, indexmapcore_2_ref_0);
    let mut usize_4: usize = crate::map::iter::IntoKeys::len(intokeys_0_ref_0);
    crate::map::core::IndexMapCore::truncate(indexmapcore_1_ref_0, usize_1);
    let mut iter_1: crate::map::iter::Iter<isize, isize> = crate::map::iter::Iter::clone(iter_0_ref_0);
    let mut valuesmut_0: crate::map::iter::ValuesMut<isize, isize> = crate::map::iter::ValuesMut::default();
    let mut iter_1_ref_0: &mut crate::map::iter::Iter<isize, isize> = &mut iter_1;
    let mut option_0: std::option::Option<(&isize, &isize)> = crate::map::iter::Iter::nth(iter_1_ref_0, usize_0);
    let mut option_1: std::option::Option<&isize> = crate::map::iter::Keys::next(keys_0_ref_0);
    let mut bucket_slice_0: &mut [crate::Bucket<isize, isize>] = crate::map::core::IndexMapCore::as_entries_mut(indexmapcore_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_47() {
    rusty_monitor::set_test_id(47);
    let mut usize_0: usize = 6341usize;
    let mut keys_0: crate::map::iter::Keys<isize, isize> = crate::map::iter::Keys::default();
    let mut keys_0_ref_0: &mut crate::map::iter::Keys<isize, isize> = &mut keys_0;
    let mut itermut2_0: crate::map::iter::IterMut2<isize, isize> = crate::map::iter::IterMut2::default();
    let mut itermut2_0_ref_0: &crate::map::iter::IterMut2<isize, isize> = &mut itermut2_0;
    let mut intovalues_0: crate::map::iter::IntoValues<isize, isize> = crate::map::iter::IntoValues::default();
    let mut intovalues_0_ref_0: &crate::map::iter::IntoValues<isize, isize> = &mut intovalues_0;
    let mut usize_1: usize = 2694usize;
    let mut iter_0: crate::set::iter::Iter<isize> = crate::set::iter::Iter::default();
    let mut iter_0_ref_0: &mut crate::set::iter::Iter<isize> = &mut iter_0;
    let mut isize_0: isize = 3644isize;
    let mut isize_1: isize = 21394isize;
    let mut isize_2: isize = -8403isize;
    let mut isize_3: isize = 18741isize;
    let mut intoiter_0: crate::map::iter::IntoIter<isize, isize> = crate::map::iter::IntoIter::default();
    let mut usize_2: usize = 6995usize;
    let mut usize_3: usize = 7894usize;
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_3);
    let mut indexmapcore_0_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_0;
    let mut usize_4: usize = 1510usize;
    let mut valuesmut_0: crate::map::iter::ValuesMut<isize, std::result::Result<(), crate::TryReserveError>> = crate::map::iter::ValuesMut::default();
    let mut valuesmut_0_ref_0: &crate::map::iter::ValuesMut<isize, std::result::Result<(), crate::TryReserveError>> = &mut valuesmut_0;
    let mut isize_4: isize = 2073isize;
    let mut isize_4_ref_0: &isize = &mut isize_4;
    let mut result_0: std::result::Result<(), crate::TryReserveError> = crate::map::core::IndexMapCore::try_reserve_exact(indexmapcore_0_ref_0, usize_2);
    let mut usize_5: usize = crate::map::iter::IntoIter::count(intoiter_0);
    let mut tuple_0: () = std::result::Result::unwrap(result_0);
    let mut option_0: std::option::Option<&isize> = crate::set::iter::Iter::nth(iter_0_ref_0, usize_1);
    let mut tuple_1: (usize, std::option::Option<usize>) = crate::map::iter::IntoValues::size_hint(intovalues_0_ref_0);
    let mut tuple_2: (usize, std::option::Option<usize>) = crate::map::iter::IterMut2::size_hint(itermut2_0_ref_0);
    let mut option_1: std::option::Option<&isize> = crate::map::iter::Keys::nth(keys_0_ref_0, usize_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_48() {
    rusty_monitor::set_test_id(48);
    let mut usize_0: usize = 5819usize;
    let mut itermut2_0: crate::map::iter::IterMut2<isize, isize> = crate::map::iter::IterMut2::default();
    let mut itermut2_0_ref_0: &mut crate::map::iter::IterMut2<isize, isize> = &mut itermut2_0;
    let mut keys_0: crate::map::iter::Keys<isize, isize> = crate::map::iter::Keys::default();
    let mut keys_0_ref_0: &crate::map::iter::Keys<isize, isize> = &mut keys_0;
    let mut isize_0: isize = 5663isize;
    let mut isize_1: isize = 2250isize;
    let mut isize_2: isize = 3996isize;
    let mut usize_1: usize = 9017usize;
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_1);
    let mut indexmapcore_0_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_0;
    let mut itermut_0: crate::map::iter::IterMut<std::string::String, std::result::Result<(), crate::TryReserveError>> = crate::map::iter::IterMut::default();
    let mut itermut_0_ref_0: &crate::map::iter::IterMut<std::string::String, std::result::Result<(), crate::TryReserveError>> = &mut itermut_0;
    let mut usize_2: usize = 3316usize;
    let mut valuesmut_0: crate::map::iter::ValuesMut<isize, isize> = crate::map::iter::ValuesMut::default();
    let mut valuesmut_0_ref_0: &mut crate::map::iter::ValuesMut<isize, isize> = &mut valuesmut_0;
    let mut usize_3: usize = 890usize;
    let mut intoiter_0: crate::map::iter::IntoIter<isize, isize> = crate::map::iter::IntoIter::default();
    let mut intoiter_0_ref_0: &mut crate::map::iter::IntoIter<isize, isize> = &mut intoiter_0;
    let mut isize_3: isize = 4266isize;
    let mut isize_4: isize = -636isize;
    let mut usize_4: usize = 1165usize;
    let mut indexmapcore_1: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_4);
    let mut indexmapcore_1_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_1;
    let mut isize_5: isize = -8074isize;
    let mut isize_6: isize = 14025isize;
    let mut option_0: std::option::Option<(isize, isize)> = crate::map::iter::IntoIter::nth(intoiter_0_ref_0, usize_3);
    let mut option_1: std::option::Option<&mut isize> = crate::map::iter::ValuesMut::nth(valuesmut_0_ref_0, usize_2);
    crate::map::core::IndexMapCore::rebuild_hash_table(indexmapcore_0_ref_0);
    let mut isize_7: &mut isize = std::option::Option::unwrap(option_1);
    let mut tuple_0: (usize, std::option::Option<usize>) = crate::map::iter::Keys::size_hint(keys_0_ref_0);
    let mut option_2: std::option::Option<(&mut isize, &mut isize)> = crate::map::iter::IterMut2::nth(itermut2_0_ref_0, usize_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_49() {
    rusty_monitor::set_test_id(49);
    let mut intoiter_0: crate::map::iter::IntoIter<isize, isize> = crate::map::iter::IntoIter::default();
    let mut intoiter_0_ref_0: &crate::map::iter::IntoIter<isize, isize> = &mut intoiter_0;
    let mut intovalues_0: crate::map::iter::IntoValues<isize, isize> = crate::map::iter::IntoValues::default();
    let mut usize_0: usize = 202usize;
    let mut indexmapcore_0: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_0_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_0;
    let mut usize_1: usize = 8345usize;
    let mut intoiter_1: crate::set::iter::IntoIter<isize> = crate::set::iter::IntoIter::default();
    let mut isize_0: isize = -2552isize;
    let mut isize_1: isize = 5568isize;
    let mut usize_2: usize = 6763usize;
    let mut indexmapcore_1: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::with_capacity(usize_2);
    let mut indexmapcore_1_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_1;
    let mut usize_3: usize = 5305usize;
    let mut usize_4: usize = 6563usize;
    let mut indexmapcore_2: crate::map::core::IndexMapCore<isize, isize> = crate::map::core::IndexMapCore::new();
    let mut indexmapcore_2_ref_0: &mut crate::map::core::IndexMapCore<isize, isize> = &mut indexmapcore_2;
    let mut usize_5: usize = 8517usize;
    let mut usize_6: usize = 3728usize;
    let mut usize_7: usize = 9961usize;
    let mut usize_8: usize = 3442usize;
    let mut iter_0: crate::map::iter::Iter<isize, isize> = crate::map::iter::Iter::default();
    let mut iter_0_ref_0: &mut crate::map::iter::Iter<isize, isize> = &mut iter_0;
    let mut option_0: std::option::Option<(&isize, &isize)> = crate::map::iter::Iter::nth(iter_0_ref_0, usize_8);
    let mut tuple_0: (&isize, &isize) = std::option::Option::unwrap(option_0);
    let mut keys_0: crate::map::iter::Keys<isize, isize> = crate::map::iter::Keys::default();
    let mut option_1: std::option::Option<(isize, isize)> = crate::map::core::IndexMapCore::swap_remove_index(indexmapcore_2_ref_0, usize_4);
    let mut option_2: std::option::Option<isize> = crate::set::iter::IntoIter::last(intoiter_1);
    let mut option_3: std::option::Option<&isize> = crate::map::iter::Keys::last(keys_0);
    let mut option_4: std::option::Option<(isize, isize)> = crate::map::core::IndexMapCore::shift_remove_index(indexmapcore_0_ref_0, usize_0);
    let mut usize_9: usize = crate::map::iter::IntoValues::count(intovalues_0);
    let mut tuple_1: (usize, std::option::Option<usize>) = crate::map::iter::IntoIter::size_hint(intoiter_0_ref_0);
    panic!("From RustyUnit with love");
}
}