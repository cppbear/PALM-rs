//! A hash set implemented using [`IndexMap`]

mod iter;
mod mutable;
mod slice;

#[cfg(test)]
mod tests;

pub use self::iter::{
    Difference, Drain, Intersection, IntoIter, Iter, Splice, SymmetricDifference, Union,
};
pub use self::mutable::MutableValues;
pub use self::slice::Slice;

#[cfg(feature = "rayon")]
pub use crate::rayon::set as rayon;
use crate::TryReserveError;

#[cfg(feature = "std")]
use std::collections::hash_map::RandomState;

use crate::util::try_simplify_range;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::fmt;
use core::hash::{BuildHasher, Hash};
use core::ops::{BitAnd, BitOr, BitXor, Index, RangeBounds, Sub};

use super::{Entries, Equivalent, IndexMap};

type Bucket<T> = super::Bucket<T, ()>;

/// A hash set where the iteration order of the values is independent of their
/// hash values.
///
/// The interface is closely compatible with the standard
/// [`HashSet`][std::collections::HashSet],
/// but also has additional features.
///
/// # Order
///
/// The values have a consistent order that is determined by the sequence of
/// insertion and removal calls on the set. The order does not depend on the
/// values or the hash function at all. Note that insertion order and value
/// are not affected if a re-insertion is attempted once an element is
/// already present.
///
/// All iterators traverse the set *in order*.  Set operation iterators like
/// [`IndexSet::union`] produce a concatenated order, as do their matching "bitwise"
/// operators.  See their documentation for specifics.
///
/// The insertion order is preserved, with **notable exceptions** like the
/// [`.remove()`][Self::remove] or [`.swap_remove()`][Self::swap_remove] methods.
/// Methods such as [`.sort_by()`][Self::sort_by] of
/// course result in a new order, depending on the sorting order.
///
/// # Indices
///
/// The values are indexed in a compact range without holes in the range
/// `0..self.len()`. For example, the method `.get_full` looks up the index for
/// a value, and the method `.get_index` looks up the value by index.
///
/// # Complexity
///
/// Internally, `IndexSet<T, S>` just holds an [`IndexMap<T, (), S>`](IndexMap). Thus the complexity
/// of the two are the same for most methods.
///
/// # Examples
///
/// ```
/// use indexmap::IndexSet;
///
/// // Collects which letters appear in a sentence.
/// let letters: IndexSet<_> = "a short treatise on fungi".chars().collect();
///
/// assert!(letters.contains(&'s'));
/// assert!(letters.contains(&'t'));
/// assert!(letters.contains(&'u'));
/// assert!(!letters.contains(&'y'));
/// ```
#[cfg(feature = "std")]
pub struct IndexSet<T, S = RandomState> {
    pub(crate) map: IndexMap<T, (), S>,
}
#[cfg(not(feature = "std"))]
pub struct IndexSet<T, S> {
    pub(crate) map: IndexMap<T, (), S>,
}

impl<T, S> Clone for IndexSet<T, S>
where
    T: Clone,
    S: Clone,
{
    fn clone(&self) -> Self {
        IndexSet {
            map: self.map.clone(),
        }
    }

    fn clone_from(&mut self, other: &Self) {
        self.map.clone_from(&other.map);
    }
}

impl<T, S> Entries for IndexSet<T, S> {
    type Entry = Bucket<T>;

    #[inline]
    fn into_entries(self) -> Vec<Self::Entry> {
        self.map.into_entries()
    }

    #[inline]
    fn as_entries(&self) -> &[Self::Entry] {
        self.map.as_entries()
    }

    #[inline]
    fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
        self.map.as_entries_mut()
    }

    fn with_entries<F>(&mut self, f: F)
    where
        F: FnOnce(&mut [Self::Entry]),
    {
        self.map.with_entries(f);
    }
}

impl<T, S> fmt::Debug for IndexSet<T, S>
where
    T: fmt::Debug,
{
    #[cfg(not(feature = "test_debug"))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }

    #[cfg(feature = "test_debug")]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Let the inner `IndexMap` print all of its details
        f.debug_struct("IndexSet").field("map", &self.map).finish()
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl<T> IndexSet<T> {
    /// Create a new set. (Does not allocate.)
    pub fn new() -> Self {
        IndexSet {
            map: IndexMap::new(),
        }
    }

    /// Create a new set with capacity for `n` elements.
    /// (Does not allocate if `n` is zero.)
    ///
    /// Computes in **O(n)** time.
    pub fn with_capacity(n: usize) -> Self {
        IndexSet {
            map: IndexMap::with_capacity(n),
        }
    }
}

impl<T, S> IndexSet<T, S> {
    /// Create a new set with capacity for `n` elements.
    /// (Does not allocate if `n` is zero.)
    ///
    /// Computes in **O(n)** time.
    pub fn with_capacity_and_hasher(n: usize, hash_builder: S) -> Self {
        IndexSet {
            map: IndexMap::with_capacity_and_hasher(n, hash_builder),
        }
    }

    /// Create a new set with `hash_builder`.
    ///
    /// This function is `const`, so it
    /// can be called in `static` contexts.
    pub const fn with_hasher(hash_builder: S) -> Self {
        IndexSet {
            map: IndexMap::with_hasher(hash_builder),
        }
    }

    /// Return the number of elements the set can hold without reallocating.
    ///
    /// This number is a lower bound; the set might be able to hold more,
    /// but is guaranteed to be able to hold at least this many.
    ///
    /// Computes in **O(1)** time.
    pub fn capacity(&self) -> usize {
        self.map.capacity()
    }

    /// Return a reference to the set's `BuildHasher`.
    pub fn hasher(&self) -> &S {
        self.map.hasher()
    }

    /// Return the number of elements in the set.
    ///
    /// Computes in **O(1)** time.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns true if the set contains no elements.
    ///
    /// Computes in **O(1)** time.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Return an iterator over the values of the set, in their order
    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(self.as_entries())
    }

    /// Remove all elements in the set, while preserving its capacity.
    ///
    /// Computes in **O(n)** time.
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Shortens the set, keeping the first `len` elements and dropping the rest.
    ///
    /// If `len` is greater than the set's current length, this has no effect.
    pub fn truncate(&mut self, len: usize) {
        self.map.truncate(len);
    }

    /// Clears the `IndexSet` in the given index range, returning those values
    /// as a drain iterator.
    ///
    /// The range may be any type that implements [`RangeBounds<usize>`],
    /// including all of the `std::ops::Range*` types, or even a tuple pair of
    /// `Bound` start and end values. To drain the set entirely, use `RangeFull`
    /// like `set.drain(..)`.
    ///
    /// This shifts down all entries following the drained range to fill the
    /// gap, and keeps the allocated memory for reuse.
    ///
    /// ***Panics*** if the starting point is greater than the end point or if
    /// the end point is greater than the length of the set.
    #[track_caller]
    pub fn drain<R>(&mut self, range: R) -> Drain<'_, T>
    where
        R: RangeBounds<usize>,
    {
        Drain::new(self.map.core.drain(range))
    }

    /// Splits the collection into two at the given index.
    ///
    /// Returns a newly allocated set containing the elements in the range
    /// `[at, len)`. After the call, the original set will be left containing
    /// the elements `[0, at)` with its previous capacity unchanged.
    ///
    /// ***Panics*** if `at > len`.
    #[track_caller]
    pub fn split_off(&mut self, at: usize) -> Self
    where
        S: Clone,
    {
        Self {
            map: self.map.split_off(at),
        }
    }

    /// Reserve capacity for `additional` more values.
    ///
    /// Computes in **O(n)** time.
    pub fn reserve(&mut self, additional: usize) {
        self.map.reserve(additional);
    }

    /// Reserve capacity for `additional` more values, without over-allocating.
    ///
    /// Unlike `reserve`, this does not deliberately over-allocate the entry capacity to avoid
    /// frequent re-allocations. However, the underlying data structures may still have internal
    /// capacity requirements, and the allocator itself may give more space than requested, so this
    /// cannot be relied upon to be precisely minimal.
    ///
    /// Computes in **O(n)** time.
    pub fn reserve_exact(&mut self, additional: usize) {
        self.map.reserve_exact(additional);
    }

    /// Try to reserve capacity for `additional` more values.
    ///
    /// Computes in **O(n)** time.
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.map.try_reserve(additional)
    }

    /// Try to reserve capacity for `additional` more values, without over-allocating.
    ///
    /// Unlike `try_reserve`, this does not deliberately over-allocate the entry capacity to avoid
    /// frequent re-allocations. However, the underlying data structures may still have internal
    /// capacity requirements, and the allocator itself may give more space than requested, so this
    /// cannot be relied upon to be precisely minimal.
    ///
    /// Computes in **O(n)** time.
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.map.try_reserve_exact(additional)
    }

    /// Shrink the capacity of the set as much as possible.
    ///
    /// Computes in **O(n)** time.
    pub fn shrink_to_fit(&mut self) {
        self.map.shrink_to_fit();
    }

    /// Shrink the capacity of the set with a lower limit.
    ///
    /// Computes in **O(n)** time.
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.map.shrink_to(min_capacity);
    }
}

impl<T, S> IndexSet<T, S>
where
    T: Hash + Eq,
    S: BuildHasher,
{
    /// Insert the value into the set.
    ///
    /// If an equivalent item already exists in the set, it returns
    /// `false` leaving the original value in the set and without
    /// altering its insertion order. Otherwise, it inserts the new
    /// item and returns `true`.
    ///
    /// Computes in **O(1)** time (amortized average).
    pub fn insert(&mut self, value: T) -> bool {
        self.map.insert(value, ()).is_none()
    }

    /// Insert the value into the set, and get its index.
    ///
    /// If an equivalent item already exists in the set, it returns
    /// the index of the existing item and `false`, leaving the
    /// original value in the set and without altering its insertion
    /// order. Otherwise, it inserts the new item and returns the index
    /// of the inserted item and `true`.
    ///
    /// Computes in **O(1)** time (amortized average).
    pub fn insert_full(&mut self, value: T) -> (usize, bool) {
        let (index, existing) = self.map.insert_full(value, ());
        (index, existing.is_none())
    }

    /// Insert the value into the set at its ordered position among sorted values.
    ///
    /// This is equivalent to finding the position with
    /// [`binary_search`][Self::binary_search], and if needed calling
    /// [`insert_before`][Self::insert_before] for a new value.
    ///
    /// If the sorted item is found in the set, it returns the index of that
    /// existing item and `false`, without any change. Otherwise, it inserts the
    /// new item and returns its sorted index and `true`.
    ///
    /// If the existing items are **not** already sorted, then the insertion
    /// index is unspecified (like [`slice::binary_search`]), but the value
    /// is moved to or inserted at that position regardless.
    ///
    /// Computes in **O(n)** time (average). Instead of repeating calls to
    /// `insert_sorted`, it may be faster to call batched [`insert`][Self::insert]
    /// or [`extend`][Self::extend] and only call [`sort`][Self::sort] or
    /// [`sort_unstable`][Self::sort_unstable] once.
    pub fn insert_sorted(&mut self, value: T) -> (usize, bool)
    where
        T: Ord,
    {
        let (index, existing) = self.map.insert_sorted(value, ());
        (index, existing.is_none())
    }

    /// Insert the value into the set before the value at the given index, or at the end.
    ///
    /// If an equivalent item already exists in the set, it returns `false` leaving the
    /// original value in the set, but moved to the new position. The returned index
    /// will either be the given index or one less, depending on how the value moved.
    /// (See [`shift_insert`](Self::shift_insert) for different behavior here.)
    ///
    /// Otherwise, it inserts the new value exactly at the given index and returns `true`.
    ///
    /// ***Panics*** if `index` is out of bounds.
    /// Valid indices are `0..=set.len()` (inclusive).
    ///
    /// Computes in **O(n)** time (average).
    ///
    /// # Examples
    ///
    /// ```
    /// use indexmap::IndexSet;
    /// let mut set: IndexSet<char> = ('a'..='z').collect();
    ///
    /// // The new value '*' goes exactly at the given index.
    /// assert_eq!(set.get_index_of(&'*'), None);
    /// assert_eq!(set.insert_before(10, '*'), (10, true));
    /// assert_eq!(set.get_index_of(&'*'), Some(10));
    ///
    /// // Moving the value 'a' up will shift others down, so this moves *before* 10 to index 9.
    /// assert_eq!(set.insert_before(10, 'a'), (9, false));
    /// assert_eq!(set.get_index_of(&'a'), Some(9));
    /// assert_eq!(set.get_index_of(&'*'), Some(10));
    ///
    /// // Moving the value 'z' down will shift others up, so this moves to exactly 10.
    /// assert_eq!(set.insert_before(10, 'z'), (10, false));
    /// assert_eq!(set.get_index_of(&'z'), Some(10));
    /// assert_eq!(set.get_index_of(&'*'), Some(11));
    ///
    /// // Moving or inserting before the endpoint is also valid.
    /// assert_eq!(set.len(), 27);
    /// assert_eq!(set.insert_before(set.len(), '*'), (26, false));
    /// assert_eq!(set.get_index_of(&'*'), Some(26));
    /// assert_eq!(set.insert_before(set.len(), '+'), (27, true));
    /// assert_eq!(set.get_index_of(&'+'), Some(27));
    /// assert_eq!(set.len(), 28);
    /// ```
    #[track_caller]
    pub fn insert_before(&mut self, index: usize, value: T) -> (usize, bool) {
        let (index, existing) = self.map.insert_before(index, value, ());
        (index, existing.is_none())
    }

    /// Insert the value into the set at the given index.
    ///
    /// If an equivalent item already exists in the set, it returns `false` leaving
    /// the original value in the set, but moved to the given index.
    /// Note that existing values **cannot** be moved to `index == set.len()`!
    /// (See [`insert_before`](Self::insert_before) for different behavior here.)
    ///
    /// Otherwise, it inserts the new value at the given index and returns `true`.
    ///
    /// ***Panics*** if `index` is out of bounds.
    /// Valid indices are `0..set.len()` (exclusive) when moving an existing value, or
    /// `0..=set.len()` (inclusive) when inserting a new value.
    ///
    /// Computes in **O(n)** time (average).
    ///
    /// # Examples
    ///
    /// ```
    /// use indexmap::IndexSet;
    /// let mut set: IndexSet<char> = ('a'..='z').collect();
    ///
    /// // The new value '*' goes exactly at the given index.
    /// assert_eq!(set.get_index_of(&'*'), None);
    /// assert_eq!(set.shift_insert(10, '*'), true);
    /// assert_eq!(set.get_index_of(&'*'), Some(10));
    ///
    /// // Moving the value 'a' up to 10 will shift others down, including the '*' that was at 10.
    /// assert_eq!(set.shift_insert(10, 'a'), false);
    /// assert_eq!(set.get_index_of(&'a'), Some(10));
    /// assert_eq!(set.get_index_of(&'*'), Some(9));
    ///
    /// // Moving the value 'z' down to 9 will shift others up, including the '*' that was at 9.
    /// assert_eq!(set.shift_insert(9, 'z'), false);
    /// assert_eq!(set.get_index_of(&'z'), Some(9));
    /// assert_eq!(set.get_index_of(&'*'), Some(10));
    ///
    /// // Existing values can move to len-1 at most, but new values can insert at the endpoint.
    /// assert_eq!(set.len(), 27);
    /// assert_eq!(set.shift_insert(set.len() - 1, '*'), false);
    /// assert_eq!(set.get_index_of(&'*'), Some(26));
    /// assert_eq!(set.shift_insert(set.len(), '+'), true);
    /// assert_eq!(set.get_index_of(&'+'), Some(27));
    /// assert_eq!(set.len(), 28);
    /// ```
    ///
    /// ```should_panic
    /// use indexmap::IndexSet;
    /// let mut set: IndexSet<char> = ('a'..='z').collect();
    ///
    /// // This is an invalid index for moving an existing value!
    /// set.shift_insert(set.len(), 'a');
    /// ```
    #[track_caller]
    pub fn shift_insert(&mut self, index: usize, value: T) -> bool {
        self.map.shift_insert(index, value, ()).is_none()
    }

    /// Adds a value to the set, replacing the existing value, if any, that is
    /// equal to the given one, without altering its insertion order. Returns
    /// the replaced value.
    ///
    /// Computes in **O(1)** time (average).
    pub fn replace(&mut self, value: T) -> Option<T> {
        self.replace_full(value).1
    }

    /// Adds a value to the set, replacing the existing value, if any, that is
    /// equal to the given one, without altering its insertion order. Returns
    /// the index of the item and its replaced value.
    ///
    /// Computes in **O(1)** time (average).
    pub fn replace_full(&mut self, value: T) -> (usize, Option<T>) {
        let hash = self.map.hash(&value);
        match self.map.core.replace_full(hash, value, ()) {
            (i, Some((replaced, ()))) => (i, Some(replaced)),
            (i, None) => (i, None),
        }
    }

    /// Return an iterator over the values that are in `self` but not `other`.
    ///
    /// Values are produced in the same order that they appear in `self`.
    pub fn difference<'a, S2>(&'a self, other: &'a IndexSet<T, S2>) -> Difference<'a, T, S2>
    where
        S2: BuildHasher,
    {
        Difference::new(self, other)
    }

    /// Return an iterator over the values that are in `self` or `other`,
    /// but not in both.
    ///
    /// Values from `self` are produced in their original order, followed by
    /// values from `other` in their original order.
    pub fn symmetric_difference<'a, S2>(
        &'a self,
        other: &'a IndexSet<T, S2>,
    ) -> SymmetricDifference<'a, T, S, S2>
    where
        S2: BuildHasher,
    {
        SymmetricDifference::new(self, other)
    }

    /// Return an iterator over the values that are in both `self` and `other`.
    ///
    /// Values are produced in the same order that they appear in `self`.
    pub fn intersection<'a, S2>(&'a self, other: &'a IndexSet<T, S2>) -> Intersection<'a, T, S2>
    where
        S2: BuildHasher,
    {
        Intersection::new(self, other)
    }

    /// Return an iterator over all values that are in `self` or `other`.
    ///
    /// Values from `self` are produced in their original order, followed by
    /// values that are unique to `other` in their original order.
    pub fn union<'a, S2>(&'a self, other: &'a IndexSet<T, S2>) -> Union<'a, T, S>
    where
        S2: BuildHasher,
    {
        Union::new(self, other)
    }

    /// Creates a splicing iterator that replaces the specified range in the set
    /// with the given `replace_with` iterator and yields the removed items.
    /// `replace_with` does not need to be the same length as `range`.
    ///
    /// The `range` is removed even if the iterator is not consumed until the
    /// end. It is unspecified how many elements are removed from the set if the
    /// `Splice` value is leaked.
    ///
    /// The input iterator `replace_with` is only consumed when the `Splice`
    /// value is dropped. If a value from the iterator matches an existing entry
    /// in the set (outside of `range`), then the original will be unchanged.
    /// Otherwise, the new value will be inserted in the replaced `range`.
    ///
    /// ***Panics*** if the starting point is greater than the end point or if
    /// the end point is greater than the length of the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use indexmap::IndexSet;
    ///
    /// let mut set = IndexSet::from([0, 1, 2, 3, 4]);
    /// let new = [5, 4, 3, 2, 1];
    /// let removed: Vec<_> = set.splice(2..4, new).collect();
    ///
    /// // 1 and 4 kept their positions, while 5, 3, and 2 were newly inserted.
    /// assert!(set.into_iter().eq([0, 1, 5, 3, 2, 4]));
    /// assert_eq!(removed, &[2, 3]);
    /// ```
    #[track_caller]
    pub fn splice<R, I>(&mut self, range: R, replace_with: I) -> Splice<'_, I::IntoIter, T, S>
    where
        R: RangeBounds<usize>,
        I: IntoIterator<Item = T>,
    {
        Splice::new(self, range, replace_with.into_iter())
    }

    /// Moves all values from `other` into `self`, leaving `other` empty.
    ///
    /// This is equivalent to calling [`insert`][Self::insert] for each value
    /// from `other` in order, which means that values that already exist
    /// in `self` are unchanged in their current position.
    ///
    /// See also [`union`][Self::union] to iterate the combined values by
    /// reference, without modifying `self` or `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use indexmap::IndexSet;
    ///
    /// let mut a = IndexSet::from([3, 2, 1]);
    /// let mut b = IndexSet::from([3, 4, 5]);
    /// let old_capacity = b.capacity();
    ///
    /// a.append(&mut b);
    ///
    /// assert_eq!(a.len(), 5);
    /// assert_eq!(b.len(), 0);
    /// assert_eq!(b.capacity(), old_capacity);
    ///
    /// assert!(a.iter().eq(&[3, 2, 1, 4, 5]));
    /// ```
    pub fn append<S2>(&mut self, other: &mut IndexSet<T, S2>) {
        self.map.append(&mut other.map);
    }
}

impl<T, S> IndexSet<T, S>
where
    S: BuildHasher,
{
    /// Return `true` if an equivalent to `value` exists in the set.
    ///
    /// Computes in **O(1)** time (average).
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {
        self.map.contains_key(value)
    }

    /// Return a reference to the value stored in the set, if it is present,
    /// else `None`.
    ///
    /// Computes in **O(1)** time (average).
    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {
        self.map.get_key_value(value).map(|(x, &())| x)
    }

    /// Return item index and value
    pub fn get_full<Q>(&self, value: &Q) -> Option<(usize, &T)>
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {
        self.map.get_full(value).map(|(i, x, &())| (i, x))
    }

    /// Return item index, if it exists in the set
    ///
    /// Computes in **O(1)** time (average).
    pub fn get_index_of<Q>(&self, value: &Q) -> Option<usize>
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {
        self.map.get_index_of(value)
    }

    /// Remove the value from the set, and return `true` if it was present.
    ///
    /// **NOTE:** This is equivalent to [`.swap_remove(value)`][Self::swap_remove], replacing this
    /// value's position with the last element, and it is deprecated in favor of calling that
    /// explicitly. If you need to preserve the relative order of the values in the set, use
    /// [`.shift_remove(value)`][Self::shift_remove] instead.
    #[deprecated(note = "`remove` disrupts the set order -- \
        use `swap_remove` or `shift_remove` for explicit behavior.")]
    pub fn remove<Q>(&mut self, value: &Q) -> bool
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {
        self.swap_remove(value)
    }

    /// Remove the value from the set, and return `true` if it was present.
    ///
    /// Like [`Vec::swap_remove`], the value is removed by swapping it with the
    /// last element of the set and popping it off. **This perturbs
    /// the position of what used to be the last element!**
    ///
    /// Return `false` if `value` was not in the set.
    ///
    /// Computes in **O(1)** time (average).
    pub fn swap_remove<Q>(&mut self, value: &Q) -> bool
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {
        self.map.swap_remove(value).is_some()
    }

    /// Remove the value from the set, and return `true` if it was present.
    ///
    /// Like [`Vec::remove`], the value is removed by shifting all of the
    /// elements that follow it, preserving their relative order.
    /// **This perturbs the index of all of those elements!**
    ///
    /// Return `false` if `value` was not in the set.
    ///
    /// Computes in **O(n)** time (average).
    pub fn shift_remove<Q>(&mut self, value: &Q) -> bool
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {
        self.map.shift_remove(value).is_some()
    }

    /// Removes and returns the value in the set, if any, that is equal to the
    /// given one.
    ///
    /// **NOTE:** This is equivalent to [`.swap_take(value)`][Self::swap_take], replacing this
    /// value's position with the last element, and it is deprecated in favor of calling that
    /// explicitly. If you need to preserve the relative order of the values in the set, use
    /// [`.shift_take(value)`][Self::shift_take] instead.
    #[deprecated(note = "`take` disrupts the set order -- \
        use `swap_take` or `shift_take` for explicit behavior.")]
    pub fn take<Q>(&mut self, value: &Q) -> Option<T>
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {
        self.swap_take(value)
    }

    /// Removes and returns the value in the set, if any, that is equal to the
    /// given one.
    ///
    /// Like [`Vec::swap_remove`], the value is removed by swapping it with the
    /// last element of the set and popping it off. **This perturbs
    /// the position of what used to be the last element!**
    ///
    /// Return `None` if `value` was not in the set.
    ///
    /// Computes in **O(1)** time (average).
    pub fn swap_take<Q>(&mut self, value: &Q) -> Option<T>
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {
        self.map.swap_remove_entry(value).map(|(x, ())| x)
    }

    /// Removes and returns the value in the set, if any, that is equal to the
    /// given one.
    ///
    /// Like [`Vec::remove`], the value is removed by shifting all of the
    /// elements that follow it, preserving their relative order.
    /// **This perturbs the index of all of those elements!**
    ///
    /// Return `None` if `value` was not in the set.
    ///
    /// Computes in **O(n)** time (average).
    pub fn shift_take<Q>(&mut self, value: &Q) -> Option<T>
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {
        self.map.shift_remove_entry(value).map(|(x, ())| x)
    }

    /// Remove the value from the set return it and the index it had.
    ///
    /// Like [`Vec::swap_remove`], the value is removed by swapping it with the
    /// last element of the set and popping it off. **This perturbs
    /// the position of what used to be the last element!**
    ///
    /// Return `None` if `value` was not in the set.
    pub fn swap_remove_full<Q>(&mut self, value: &Q) -> Option<(usize, T)>
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {
        self.map.swap_remove_full(value).map(|(i, x, ())| (i, x))
    }

    /// Remove the value from the set return it and the index it had.
    ///
    /// Like [`Vec::remove`], the value is removed by shifting all of the
    /// elements that follow it, preserving their relative order.
    /// **This perturbs the index of all of those elements!**
    ///
    /// Return `None` if `value` was not in the set.
    pub fn shift_remove_full<Q>(&mut self, value: &Q) -> Option<(usize, T)>
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {
        self.map.shift_remove_full(value).map(|(i, x, ())| (i, x))
    }
}

impl<T, S> IndexSet<T, S> {
    /// Remove the last value
    ///
    /// This preserves the order of the remaining elements.
    ///
    /// Computes in **O(1)** time (average).
    #[doc(alias = "pop_last")] // like `BTreeSet`
    pub fn pop(&mut self) -> Option<T> {
        self.map.pop().map(|(x, ())| x)
    }

    /// Scan through each value in the set and keep those where the
    /// closure `keep` returns `true`.
    ///
    /// The elements are visited in order, and remaining elements keep their
    /// order.
    ///
    /// Computes in **O(n)** time (average).
    pub fn retain<F>(&mut self, mut keep: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.map.retain(move |x, &mut ()| keep(x))
    }

    /// Sort the set’s values by their default ordering.
    ///
    /// This is a stable sort -- but equivalent values should not normally coexist in
    /// a set at all, so [`sort_unstable`][Self::sort_unstable] is preferred
    /// because it is generally faster and doesn't allocate auxiliary memory.
    ///
    /// See [`sort_by`](Self::sort_by) for details.
    pub fn sort(&mut self)
    where
        T: Ord,
    {
        self.map.sort_keys()
    }

    /// Sort the set’s values in place using the comparison function `cmp`.
    ///
    /// Computes in **O(n log n)** time and **O(n)** space. The sort is stable.
    pub fn sort_by<F>(&mut self, mut cmp: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.map.sort_by(move |a, _, b, _| cmp(a, b));
    }

    /// Sort the values of the set and return a by-value iterator of
    /// the values with the result.
    ///
    /// The sort is stable.
    pub fn sorted_by<F>(self, mut cmp: F) -> IntoIter<T>
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        let mut entries = self.into_entries();
        entries.sort_by(move |a, b| cmp(&a.key, &b.key));
        IntoIter::new(entries)
    }

    /// Sort the set's values by their default ordering.
    ///
    /// See [`sort_unstable_by`](Self::sort_unstable_by) for details.
    pub fn sort_unstable(&mut self)
    where
        T: Ord,
    {
        self.map.sort_unstable_keys()
    }

    /// Sort the set's values in place using the comparison function `cmp`.
    ///
    /// Computes in **O(n log n)** time. The sort is unstable.
    pub fn sort_unstable_by<F>(&mut self, mut cmp: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.map.sort_unstable_by(move |a, _, b, _| cmp(a, b))
    }

    /// Sort the values of the set and return a by-value iterator of
    /// the values with the result.
    pub fn sorted_unstable_by<F>(self, mut cmp: F) -> IntoIter<T>
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        let mut entries = self.into_entries();
        entries.sort_unstable_by(move |a, b| cmp(&a.key, &b.key));
        IntoIter::new(entries)
    }

    /// Sort the set’s values in place using a key extraction function.
    ///
    /// During sorting, the function is called at most once per entry, by using temporary storage
    /// to remember the results of its evaluation. The order of calls to the function is
    /// unspecified and may change between versions of `indexmap` or the standard library.
    ///
    /// Computes in **O(m n + n log n + c)** time () and **O(n)** space, where the function is
    /// **O(m)**, *n* is the length of the map, and *c* the capacity. The sort is stable.
    pub fn sort_by_cached_key<K, F>(&mut self, mut sort_key: F)
    where
        K: Ord,
        F: FnMut(&T) -> K,
    {
        self.with_entries(move |entries| {
            entries.sort_by_cached_key(move |a| sort_key(&a.key));
        });
    }

    /// Search over a sorted set for a value.
    ///
    /// Returns the position where that value is present, or the position where it can be inserted
    /// to maintain the sort. See [`slice::binary_search`] for more details.
    ///
    /// Computes in **O(log(n))** time, which is notably less scalable than looking the value up
    /// using [`get_index_of`][IndexSet::get_index_of], but this can also position missing values.
    pub fn binary_search(&self, x: &T) -> Result<usize, usize>
    where
        T: Ord,
    {
        self.as_slice().binary_search(x)
    }

    /// Search over a sorted set with a comparator function.
    ///
    /// Returns the position where that value is present, or the position where it can be inserted
    /// to maintain the sort. See [`slice::binary_search_by`] for more details.
    ///
    /// Computes in **O(log(n))** time.
    #[inline]
    pub fn binary_search_by<'a, F>(&'a self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a T) -> Ordering,
    {
        self.as_slice().binary_search_by(f)
    }

    /// Search over a sorted set with an extraction function.
    ///
    /// Returns the position where that value is present, or the position where it can be inserted
    /// to maintain the sort. See [`slice::binary_search_by_key`] for more details.
    ///
    /// Computes in **O(log(n))** time.
    #[inline]
    pub fn binary_search_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a T) -> B,
        B: Ord,
    {
        self.as_slice().binary_search_by_key(b, f)
    }

    /// Returns the index of the partition point of a sorted set according to the given predicate
    /// (the index of the first element of the second partition).
    ///
    /// See [`slice::partition_point`] for more details.
    ///
    /// Computes in **O(log(n))** time.
    #[must_use]
    pub fn partition_point<P>(&self, pred: P) -> usize
    where
        P: FnMut(&T) -> bool,
    {
        self.as_slice().partition_point(pred)
    }

    /// Reverses the order of the set’s values in place.
    ///
    /// Computes in **O(n)** time and **O(1)** space.
    pub fn reverse(&mut self) {
        self.map.reverse()
    }

    /// Returns a slice of all the values in the set.
    ///
    /// Computes in **O(1)** time.
    pub fn as_slice(&self) -> &Slice<T> {
        Slice::from_slice(self.as_entries())
    }

    /// Converts into a boxed slice of all the values in the set.
    ///
    /// Note that this will drop the inner hash table and any excess capacity.
    pub fn into_boxed_slice(self) -> Box<Slice<T>> {
        Slice::from_boxed(self.into_entries().into_boxed_slice())
    }

    /// Get a value by index
    ///
    /// Valid indices are `0 <= index < self.len()`.
    ///
    /// Computes in **O(1)** time.
    pub fn get_index(&self, index: usize) -> Option<&T> {
        self.as_entries().get(index).map(Bucket::key_ref)
    }

    /// Returns a slice of values in the given range of indices.
    ///
    /// Valid indices are `0 <= index < self.len()`.
    ///
    /// Computes in **O(1)** time.
    pub fn get_range<R: RangeBounds<usize>>(&self, range: R) -> Option<&Slice<T>> {
        let entries = self.as_entries();
        let range = try_simplify_range(range, entries.len())?;
        entries.get(range).map(Slice::from_slice)
    }

    /// Get the first value
    ///
    /// Computes in **O(1)** time.
    pub fn first(&self) -> Option<&T> {
        self.as_entries().first().map(Bucket::key_ref)
    }

    /// Get the last value
    ///
    /// Computes in **O(1)** time.
    pub fn last(&self) -> Option<&T> {
        self.as_entries().last().map(Bucket::key_ref)
    }

    /// Remove the value by index
    ///
    /// Valid indices are `0 <= index < self.len()`.
    ///
    /// Like [`Vec::swap_remove`], the value is removed by swapping it with the
    /// last element of the set and popping it off. **This perturbs
    /// the position of what used to be the last element!**
    ///
    /// Computes in **O(1)** time (average).
    pub fn swap_remove_index(&mut self, index: usize) -> Option<T> {
        self.map.swap_remove_index(index).map(|(x, ())| x)
    }

    /// Remove the value by index
    ///
    /// Valid indices are `0 <= index < self.len()`.
    ///
    /// Like [`Vec::remove`], the value is removed by shifting all of the
    /// elements that follow it, preserving their relative order.
    /// **This perturbs the index of all of those elements!**
    ///
    /// Computes in **O(n)** time (average).
    pub fn shift_remove_index(&mut self, index: usize) -> Option<T> {
        self.map.shift_remove_index(index).map(|(x, ())| x)
    }

    /// Moves the position of a value from one index to another
    /// by shifting all other values in-between.
    ///
    /// * If `from < to`, the other values will shift down while the targeted value moves up.
    /// * If `from > to`, the other values will shift up while the targeted value moves down.
    ///
    /// ***Panics*** if `from` or `to` are out of bounds.
    ///
    /// Computes in **O(n)** time (average).
    #[track_caller]
    pub fn move_index(&mut self, from: usize, to: usize) {
        self.map.move_index(from, to)
    }

    /// Swaps the position of two values in the set.
    ///
    /// ***Panics*** if `a` or `b` are out of bounds.
    ///
    /// Computes in **O(1)** time (average).
    #[track_caller]
    pub fn swap_indices(&mut self, a: usize, b: usize) {
        self.map.swap_indices(a, b)
    }
}

/// Access [`IndexSet`] values at indexed positions.
///
/// # Examples
///
/// ```
/// use indexmap::IndexSet;
///
/// let mut set = IndexSet::new();
/// for word in "Lorem ipsum dolor sit amet".split_whitespace() {
///     set.insert(word.to_string());
/// }
/// assert_eq!(set[0], "Lorem");
/// assert_eq!(set[1], "ipsum");
/// set.reverse();
/// assert_eq!(set[0], "amet");
/// assert_eq!(set[1], "sit");
/// set.sort();
/// assert_eq!(set[0], "Lorem");
/// assert_eq!(set[1], "amet");
/// ```
///
/// ```should_panic
/// use indexmap::IndexSet;
///
/// let mut set = IndexSet::new();
/// set.insert("foo");
/// println!("{:?}", set[10]); // panics!
/// ```
impl<T, S> Index<usize> for IndexSet<T, S> {
    type Output = T;

    /// Returns a reference to the value at the supplied `index`.
    ///
    /// ***Panics*** if `index` is out of bounds.
    fn index(&self, index: usize) -> &T {
        self.get_index(index).unwrap_or_else(|| {
            panic!(
                "index out of bounds: the len is {len} but the index is {index}",
                len = self.len()
            );
        })
    }
}

impl<T, S> FromIterator<T> for IndexSet<T, S>
where
    T: Hash + Eq,
    S: BuildHasher + Default,
{
    fn from_iter<I: IntoIterator<Item = T>>(iterable: I) -> Self {
        let iter = iterable.into_iter().map(|x| (x, ()));
        IndexSet {
            map: IndexMap::from_iter(iter),
        }
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl<T, const N: usize> From<[T; N]> for IndexSet<T, RandomState>
where
    T: Eq + Hash,
{
    /// # Examples
    ///
    /// ```
    /// use indexmap::IndexSet;
    ///
    /// let set1 = IndexSet::from([1, 2, 3, 4]);
    /// let set2: IndexSet<_> = [1, 2, 3, 4].into();
    /// assert_eq!(set1, set2);
    /// ```
    fn from(arr: [T; N]) -> Self {
        Self::from_iter(arr)
    }
}

impl<T, S> Extend<T> for IndexSet<T, S>
where
    T: Hash + Eq,
    S: BuildHasher,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iterable: I) {
        let iter = iterable.into_iter().map(|x| (x, ()));
        self.map.extend(iter);
    }
}

impl<'a, T, S> Extend<&'a T> for IndexSet<T, S>
where
    T: Hash + Eq + Copy + 'a,
    S: BuildHasher,
{
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iterable: I) {
        let iter = iterable.into_iter().copied();
        self.extend(iter);
    }
}

impl<T, S> Default for IndexSet<T, S>
where
    S: Default,
{
    /// Return an empty [`IndexSet`]
    fn default() -> Self {
        IndexSet {
            map: IndexMap::default(),
        }
    }
}

impl<T, S1, S2> PartialEq<IndexSet<T, S2>> for IndexSet<T, S1>
where
    T: Hash + Eq,
    S1: BuildHasher,
    S2: BuildHasher,
{
    fn eq(&self, other: &IndexSet<T, S2>) -> bool {
        self.len() == other.len() && self.is_subset(other)
    }
}

impl<T, S> Eq for IndexSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
}

impl<T, S> IndexSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    /// Returns `true` if `self` has no elements in common with `other`.
    pub fn is_disjoint<S2>(&self, other: &IndexSet<T, S2>) -> bool
    where
        S2: BuildHasher,
    {
        if self.len() <= other.len() {
            self.iter().all(move |value| !other.contains(value))
        } else {
            other.iter().all(move |value| !self.contains(value))
        }
    }

    /// Returns `true` if all elements of `self` are contained in `other`.
    pub fn is_subset<S2>(&self, other: &IndexSet<T, S2>) -> bool
    where
        S2: BuildHasher,
    {
        self.len() <= other.len() && self.iter().all(move |value| other.contains(value))
    }

    /// Returns `true` if all elements of `other` are contained in `self`.
    pub fn is_superset<S2>(&self, other: &IndexSet<T, S2>) -> bool
    where
        S2: BuildHasher,
    {
        other.is_subset(self)
    }
}

impl<T, S1, S2> BitAnd<&IndexSet<T, S2>> for &IndexSet<T, S1>
where
    T: Eq + Hash + Clone,
    S1: BuildHasher + Default,
    S2: BuildHasher,
{
    type Output = IndexSet<T, S1>;

    /// Returns the set intersection, cloned into a new set.
    ///
    /// Values are collected in the same order that they appear in `self`.
    fn bitand(self, other: &IndexSet<T, S2>) -> Self::Output {
        self.intersection(other).cloned().collect()
    }
}

impl<T, S1, S2> BitOr<&IndexSet<T, S2>> for &IndexSet<T, S1>
where
    T: Eq + Hash + Clone,
    S1: BuildHasher + Default,
    S2: BuildHasher,
{
    type Output = IndexSet<T, S1>;

    /// Returns the set union, cloned into a new set.
    ///
    /// Values from `self` are collected in their original order, followed by
    /// values that are unique to `other` in their original order.
    fn bitor(self, other: &IndexSet<T, S2>) -> Self::Output {
        self.union(other).cloned().collect()
    }
}

impl<T, S1, S2> BitXor<&IndexSet<T, S2>> for &IndexSet<T, S1>
where
    T: Eq + Hash + Clone,
    S1: BuildHasher + Default,
    S2: BuildHasher,
{
    type Output = IndexSet<T, S1>;

    /// Returns the set symmetric-difference, cloned into a new set.
    ///
    /// Values from `self` are collected in their original order, followed by
    /// values from `other` in their original order.
    fn bitxor(self, other: &IndexSet<T, S2>) -> Self::Output {
        self.symmetric_difference(other).cloned().collect()
    }
}

impl<T, S1, S2> Sub<&IndexSet<T, S2>> for &IndexSet<T, S1>
where
    T: Eq + Hash + Clone,
    S1: BuildHasher + Default,
    S2: BuildHasher,
{
    type Output = IndexSet<T, S1>;

    /// Returns the set difference, cloned into a new set.
    ///
    /// Values are collected in the same order that they appear in `self`.
    fn sub(self, other: &IndexSet<T, S2>) -> Self::Output {
        self.difference(other).cloned().collect()
    }
}


#[cfg(test)]
mod tests_llm_16_6 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_bitand_intersection() {
        let set1: IndexSet<u32> = vec![1, 2, 3].into_iter().collect();
        let set2: IndexSet<u32> = vec![3, 4, 5].into_iter().collect();
        let result: IndexSet<u32> = set1.bitand(&set2);
        let expected: IndexSet<u32> = vec![3].into_iter().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bitand_empty_intersection() {
        let set1: IndexSet<u32> = vec![1, 2, 3].into_iter().collect();
        let set2: IndexSet<u32> = vec![4, 5, 6].into_iter().collect();
        let result: IndexSet<u32> = set1.bitand(&set2);
        let expected: IndexSet<u32> = IndexSet::new();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bitand_self_intersection() {
        let set: IndexSet<u32> = vec![1, 2, 3].into_iter().collect();
        let result: IndexSet<u32> = set.bitand(&set);
        let expected: IndexSet<u32> = vec![1, 2, 3].into_iter().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bitand_with_empty_set() {
        let set1: IndexSet<u32> = vec![1, 2, 3].into_iter().collect();
        let set2: IndexSet<u32> = IndexSet::new();
        let result: IndexSet<u32> = set1.bitand(&set2);
        let expected: IndexSet<u32> = IndexSet::new();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bitand_with_different_types() {
        let set1: IndexSet<char> = vec!['a', 'b', 'c'].into_iter().collect();
        let set2: IndexSet<char> = vec!['b', 'c', 'd'].into_iter().collect();
        let result: IndexSet<char> = set1.bitand(&set2);
        let expected: IndexSet<char> = vec!['b', 'c'].into_iter().collect();
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod tests_llm_16_7 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_bitor() {
        let mut set1: IndexSet<i32> = IndexSet::new();
        set1.insert(1);
        set1.insert(2);
        set1.insert(3);
        
        let mut set2: IndexSet<i32> = IndexSet::new();
        set2.insert(3);
        set2.insert(4);
        set2.insert(5);
        
        let result = set1.bitor(&set2);
        
        // Check that the result is the union of the two sets
        let expected: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3, 4, 5]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bitor_empty() {
        let set1: IndexSet<i32> = IndexSet::new();
        let set2: IndexSet<i32> = IndexSet::new();
        
        let result = set1.bitor(&set2);
        
        // Check that the result is an empty set
        let expected: IndexSet<i32> = IndexSet::new();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bitor_disjoint() {
        let mut set1: IndexSet<i32> = IndexSet::new();
        set1.insert(1);
        set1.insert(2);
        
        let mut set2: IndexSet<i32> = IndexSet::new();
        set2.insert(3);
        set2.insert(4);
        
        let result = set1.bitor(&set2);
        
        // Check that the result is the union of the two sets
        let expected: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3, 4]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bitor_with_duplicates() {
        let mut set1: IndexSet<i32> = IndexSet::new();
        set1.insert(1);
        set1.insert(2);
        set1.insert(3);
        
        let mut set2: IndexSet<i32> = IndexSet::new();
        set2.insert(3);
        set2.insert(2);  // Duplicate
        set2.insert(4);
        
        let result = set1.bitor(&set2);
        
        // Check that the result is the union of the two sets
        let expected: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3, 4]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bitor_order() {
        let mut set1: IndexSet<i32> = IndexSet::new();
        set1.insert(1);
        set1.insert(3);
        
        let mut set2: IndexSet<i32> = IndexSet::new();
        set2.insert(2);
        set2.insert(3);  // Overlap
        
        let result = set1.bitor(&set2);
        
        // Check that the order is preserved
        let expected: IndexSet<i32> = IndexSet::from_iter(vec![1, 3, 2]);
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod tests_llm_16_8 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_bitxor_sym_diff() {
        let set1: IndexSet<u32> = (1..=5).collect();
        let set2: IndexSet<u32> = (4..=8).collect();

        let result = set1.bitxor(&set2);

        let expected: IndexSet<u32> = (1..=3).chain(5..=8).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bitxor_disjoint_sets() {
        let set1: IndexSet<char> = vec!['a', 'b', 'c'].into_iter().collect();
        let set2: IndexSet<char> = vec!['d', 'e', 'f'].into_iter().collect();

        let result = set1.bitxor(&set2);

        let expected: IndexSet<char> = vec!['a', 'b', 'c', 'd', 'e', 'f'].into_iter().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bitxor_empty_set() {
        let set1: IndexSet<i32> = IndexSet::new();
        let set2: IndexSet<i32> = (1..=5).collect();

        let result = set1.bitxor(&set2);
        assert_eq!(result, set2);
    }

    #[test]
    fn test_bitxor_identical_sets() {
        let set1: IndexSet<i32> = (1..=5).collect();
        let result = set1.bitxor(&set1);
        assert_eq!(result, IndexSet::new());
    }

    #[test]
    fn test_bitxor_order_preservation() {
        let set1: IndexSet<char> = vec!['a', 'b', 'c'].into_iter().collect();
        let set2: IndexSet<char> = vec!['c', 'd', 'e'].into_iter().collect();

        let result = set1.bitxor(&set2);
        let expected: IndexSet<char> = vec!['a', 'b', 'd', 'e'].into_iter().collect();
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod tests_llm_16_9 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_sub() {
        let set1: IndexSet<i32> = vec![1, 2, 3].into_iter().collect();
        let set2: IndexSet<i32> = vec![2, 3, 4].into_iter().collect();
        
        let result = set1.sub(&set2);
        
        let expected: IndexSet<i32> = vec![1].into_iter().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sub_empty() {
        let set1: IndexSet<i32> = vec![1, 2, 3].into_iter().collect();
        let set2: IndexSet<i32> = IndexSet::new();
        
        let result = set1.sub(&set2);
        
        let expected = set1.clone();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sub_no_difference() {
        let set1: IndexSet<i32> = vec![2, 3, 4].into_iter().collect();
        let set2: IndexSet<i32> = vec![2, 3, 4].into_iter().collect();
        
        let result = set1.sub(&set2);
        
        let expected: IndexSet<i32> = IndexSet::new();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sub_some_difference() {
        let set1: IndexSet<i32> = vec![1, 2, 3, 4].into_iter().collect();
        let set2: IndexSet<i32> = vec![2, 4].into_iter().collect();
        
        let result = set1.sub(&set2);
        
        let expected: IndexSet<i32> = vec![1, 3].into_iter().collect();
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod tests_llm_16_206 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_clone() {
        let mut original = IndexMap::new();
        original.insert(1, "one");
        original.insert(2, "two");

        let cloned = original.clone();

        assert_eq!(original.len(), cloned.len());
        assert_eq!(cloned.get(&1), Some(&"one"));
        assert_eq!(cloned.get(&2), Some(&"two"));
        assert!(cloned != original); // Ensure cloning gives a separate instance
    }

    #[test]
    fn test_clone_empty() {
        let original: IndexMap<i32, &str> = IndexMap::new();
        let cloned = original.clone();
        
        assert_eq!(original.len(), cloned.len());
        assert_eq!(cloned.len(), 0);
    }

    #[test]
    fn test_clone_with_different_values() {
        let mut original = IndexMap::new();
        original.insert(1, "one");
        original.insert(2, "two");

        let cloned = original.clone();
        original.insert(3, "three");

        assert_eq!(original.len(), 3);
        assert_eq!(cloned.len(), 2);
        assert_eq!(cloned.get(&3), None);
    }

    #[test]
    fn test_clone_trait_implementation() {
        let mut original = IndexMap::new();
        original.insert(1, "one");
        original.insert(2, "two");

        let cloned: IndexMap<i32, &str> = original.clone();
        
        assert_eq!(original, cloned);
    }
}

#[cfg(test)]
mod tests_llm_16_207 {
    use super::*; // Import everything from the parent module

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_clone_from() {
        let mut map_a = IndexMap::new();
        map_a.insert(1, "one");
        map_a.insert(2, "two");

        let mut map_b = IndexMap::new();
        map_b.insert(3, "three");
        map_b.insert(4, "four");

        // Test clone_from
        map_a.clone_from(&map_b);

        // Verify that map_a now includes entries from map_b
        assert_eq!(map_a.len(), 2);
        assert_eq!(map_a.get(&3), Some(&"three"));
        assert_eq!(map_a.get(&4), Some(&"four"));
    }

    #[test]
    fn test_clone_from_with_partial_data() {
        let mut map_a = IndexMap::new();
        map_a.insert(1, "one");

        let mut map_b = IndexMap::new();
        map_b.insert(2, "two");

        // Test clone_from
        map_a.clone_from(&map_b);

        // Verify that map_a now includes entries from map_b
        assert_eq!(map_a.len(), 1);
        assert_eq!(map_a.get(&2), Some(&"two"));
    }

    #[test]
    fn test_clone_from_empty() {
        let mut map_a = IndexMap::new();

        let mut map_b = IndexMap::new();
        map_b.insert(1, "one");

        // Test clone_from
        map_a.clone_from(&map_b);

        // Verify map_a is now empty as it should not retain previous state
        assert_eq!(map_a.len(), 0);
        assert_eq!(map_b.len(), 1);
    }

    #[test]
    fn test_clone_from_no_overlap() {
        let mut map_a = IndexMap::new();
        map_a.insert(1, "one");

        let mut map_b = IndexMap::new();
        map_b.insert(2, "two");

        // Test clone_from
        map_a.clone_from(&map_b);

        // Verify that map_a does not retain the first entry
        assert_eq!(map_a.len(), 1);
        assert_eq!(map_a.get(&2), Some(&"two"));
    }

    #[test]
    fn test_clone_from_identical_maps() {
        let mut map_a = IndexMap::new();
        map_a.insert(1, "one");

        let mut map_b = map_a.clone();

        // Test clone_from
        map_a.clone_from(&map_b);

        // Verify that map_a is unchanged
        assert_eq!(map_a.len(), 1);
        assert_eq!(map_a.get(&1), Some(&"one"));
    }
}

#[cfg(test)]
mod tests_llm_16_208 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_index_set_default() {
        let set: IndexSet<u32> = IndexSet::default();

        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_index_set_default_multiple_creations() {
        let set1: IndexSet<u32> = IndexSet::default();
        let set2: IndexSet<u32> = IndexSet::default();

        assert!(set1.is_empty());
        assert!(set2.is_empty());
        assert_eq!(set1.len(), 0);
        assert_eq!(set2.len(), 0);
    }
}

#[cfg(test)]
mod tests_llm_16_211 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_extend_with_slice() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.extend([1, 2, 3]);

        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn test_extend_with_vec() {
        let mut set: IndexSet<i32> = IndexSet::new();
        let vec = vec![4, 5, 6];
        set.extend(vec);

        assert!(set.contains(&4));
        assert!(set.contains(&5));
        assert!(set.contains(&6));
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn test_extend_with_iter() {
        let mut set: IndexSet<i32> = IndexSet::new();
        let iter = (7..10).into_iter();
        set.extend(iter);

        assert!(set.contains(&7));
        assert!(set.contains(&8));
        assert!(set.contains(&9));
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn test_extend_with_duplicates() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.extend([1, 2, 3]);
        set.extend([2, 3, 4]);

        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
        assert!(set.contains(&4));
        assert_eq!(set.len(), 4);
    }

    #[test]
    fn test_extend_with_empty() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.extend(Vec::<i32>::new());

        assert!(set.is_empty());
    }
}

#[cfg(test)]
mod tests_llm_16_212 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_from_iter_empty() {
        let set: IndexSet<i32> = IndexSet::from_iter(vec![]);
        assert!(set.is_empty());
    }

    #[test]
    fn test_from_iter_single() {
        let set: IndexSet<i32> = IndexSet::from_iter(vec![1]);
        assert_eq!(set.len(), 1);
        assert!(set.contains(&1));
    }

    #[test]
    fn test_from_iter_multiple() {
        let set: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3, 4]);
        assert_eq!(set.len(), 4);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
        assert!(set.contains(&4));
    }

    #[test]
    fn test_from_iter_duplicates() {
        let set: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 2, 3]);
        assert_eq!(set.len(), 3);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
    }

    #[test]
    fn test_from_iter_with_mixed_elements() {
        let set: IndexSet<&str> = IndexSet::from_iter(vec!["a", "b", "c", "a", "b"]);
        assert_eq!(set.len(), 3);
        assert!(set.contains("a"));
        assert!(set.contains("b"));
        assert!(set.contains("c"));
    }
}

#[cfg(test)]
mod tests_llm_16_213 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_index_valid() {
        let mut index_set: IndexSet<i32> = IndexSet::new();
        index_set.insert(1);
        index_set.insert(2);
        index_set.insert(3);

        assert_eq!(*index_set.index(0), 1);
        assert_eq!(*index_set.index(1), 2);
        assert_eq!(*index_set.index(2), 3);
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 3 but the index is 3")]
    fn test_index_out_of_bounds() {
        let index_set: IndexSet<i32> = IndexSet::new();
        index_set.index(0);  // This is out of bounds
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
    fn test_index_empty() {
        let index_set: IndexSet<i32> = IndexSet::new();
        index_set.index(0);  // This is out of bounds
    }

    #[test]
    fn test_index_boundary_conditions() {
        let mut index_set: IndexSet<i32> = IndexSet::new();
        index_set.insert(10);
        assert_eq!(*index_set.index(0), 10);
        assert_eq!(index_set.len(), 1);
    }
}

#[cfg(test)]
mod tests_llm_16_217 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_index_set_from_array() {
        let set_from_array: IndexSet<i32> = IndexSet::from([1, 2, 3, 4]);
        let set_from_slice: IndexSet<i32> = [1, 2, 3, 4].into();
        
        assert_eq!(set_from_array, set_from_slice);
    }

    #[test]
    fn test_index_set_from_array_empty() {
        let set_from_array: IndexSet<i32> = IndexSet::from([]);
        let set_from_slice: IndexSet<i32> = [].into();
        
        assert_eq!(set_from_array, set_from_slice);
    }

    #[test]
    fn test_index_set_from_array_repeats() {
        let set_from_array: IndexSet<i32> = IndexSet::from([1, 1, 2, 2, 3]);
        let set_from_slice: IndexSet<i32> = [1, 1, 2, 2, 3].into();
        
        assert_eq!(set_from_array, set_from_slice);
        assert_eq!(set_from_array.len(), 3);
    }
}

#[cfg(test)]
mod tests_llm_16_607 {
    use crate::IndexSet;

    #[test]
    fn test_append() {
        let mut a = IndexSet::from([3, 2, 1]);
        let mut b = IndexSet::from([3, 4, 5]);
        let old_capacity = b.capacity();

        a.append(&mut b);

        assert_eq!(a.len(), 5);
        assert_eq!(b.len(), 0);
        assert_eq!(b.capacity(), old_capacity);
        assert!(a.iter().eq(&[3, 2, 1, 4, 5]));
    }

    #[test]
    fn test_append_empty() {
        let mut a = IndexSet::from([1, 2, 3]);
        let mut b: IndexSet<u32> = IndexSet::new();

        a.append(&mut b);

        assert_eq!(a.len(), 3);
        assert_eq!(b.len(), 0);
    }

    #[test]
    fn test_append_existing_entries() {
        let mut a = IndexSet::from([1, 2, 3]);
        let mut b = IndexSet::from([2, 3, 4]);
        let old_capacity = b.capacity();

        a.append(&mut b);

        assert_eq!(a.len(), 4);
        assert_eq!(b.len(), 0);
        assert_eq!(b.capacity(), old_capacity);
        assert!(a.iter().eq(&[1, 2, 3, 4]));
    }

    #[test]
    fn test_append_persisted_order() {
        let mut a = IndexSet::from([1, 3, 5]);
        let mut b = IndexSet::from([2, 4, 4]); // Note duplicated "4"

        a.append(&mut b);

        assert_eq!(a.len(), 5);
        assert!(a.iter().eq(&[1, 3, 5, 2, 4]));
    }
}

#[cfg(test)]
mod tests_llm_16_608 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_as_slice() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        let slice = set.as_slice();
        assert_eq!(slice.len(), 3);
        assert_eq!(slice[0], 1);
        assert_eq!(slice[1], 2);
        assert_eq!(slice[2], 3);
    }

    #[test]
    fn test_as_slice_empty() {
        let set: IndexSet<i32> = IndexSet::new();
        let slice = set.as_slice();
        assert_eq!(slice.len(), 0);
    }

    #[test]
    fn test_as_slice_duplicates() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(1);
        set.insert(1);
        set.insert(2);
        
        let slice = set.as_slice();
        assert_eq!(slice.len(), 2);
        assert_eq!(slice[0], 1);
        assert_eq!(slice[1], 2);
    }
}

#[cfg(test)]
mod tests_llm_16_609 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_binary_search_found() {
        let mut set = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        let result = set.binary_search(&2);
        assert_eq!(result, Ok(1));
    }

    #[test]
    fn test_binary_search_not_found() {
        let mut set = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        let result = set.binary_search(&4);
        assert_eq!(result, Err(3));
    }

    #[test]
    fn test_binary_search_empty() {
        let set: IndexSet<i32> = IndexSet::new();
        let result = set.binary_search(&1);
        assert_eq!(result, Err(0));
    }

    #[test]
    fn test_binary_search_multiple_inserts() {
        let mut set = IndexSet::new();
        set.insert(3);
        set.insert(1);
        set.insert(2);

        let result = set.binary_search(&1);
        assert_eq!(result, Ok(0));
    }

    #[test]
    fn test_binary_search_with_duplicates() {
        let mut set = IndexSet::new();
        set.insert(1);
        set.insert(1);
        set.insert(1);

        let result = set.binary_search(&1);
        assert_eq!(result, Ok(0));
    }
}

#[cfg(test)]
mod tests_llm_16_613 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_clear_on_empty_index_set() {
        let mut index_set: IndexSet<i32> = IndexSet::new();
        index_set.clear();
        assert!(index_set.is_empty());
    }

    #[test]
    fn test_clear_on_non_empty_index_set() {
        let mut index_set: IndexSet<i32> = IndexSet::new();
        index_set.insert(1);
        index_set.insert(2);
        index_set.clear();
        assert!(index_set.is_empty());
    }

    #[test]
    fn test_clear_preserves_capacity() {
        let mut index_set: IndexSet<i32> = IndexSet::with_capacity(10);
        index_set.insert(1);
        index_set.insert(2);
        let capacity_before = index_set.capacity();
        index_set.clear();
        assert_eq!(index_set.capacity(), capacity_before);
    }

    #[test]
    fn test_clear_does_not_panic() {
        let mut index_set: IndexSet<i32> = IndexSet::new();
        index_set.clear(); // should not panic
    }
}

#[cfg(test)]
mod tests_llm_16_614 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_contains() {
        // Creating an IndexSet
        let mut set: IndexSet<i32> = IndexSet::new();

        // Adding elements to the set
        set.insert(1);
        set.insert(2);
        set.insert(3);

        // Test cases
        assert!(set.contains(&1)); // should return true
        assert!(set.contains(&2)); // should return true
        assert!(set.contains(&3)); // should return true
        assert!(!set.contains(&4)); // should return false
        assert!(!set.contains(&0)); // should return false
    }

    #[test]
    fn test_contains_empty_set() {
        // Creating an empty IndexSet
        let set: IndexSet<i32> = IndexSet::new();

        // Test cases
        assert!(!set.contains(&1)); // should return false
        assert!(!set.contains(&2)); // should return false
    }

    #[test]
    fn test_contains_with_different_types() {
        // Creating an IndexSet for strings
        let mut set: IndexSet<&str> = IndexSet::new();

        // Adding elements to the set
        set.insert("hello");
        set.insert("world");

        // Test cases
        assert!(set.contains(&"hello")); // should return true
        assert!(set.contains(&"world")); // should return true
        assert!(!set.contains(&"rust")); // should return false
    }
}

#[cfg(test)]
mod tests_llm_16_616 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    #[should_panic]
    fn test_drain_panic_start_greater_than_end() {
        let mut set = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.drain(1..0); // This should panic
    }

    #[test]
    #[should_panic]
    fn test_drain_panic_end_greater_than_length() {
        let mut set = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.drain(0..3); // This should panic
    }

    #[test]
    fn test_drain_entire_set() {
        let mut set = IndexSet::new();
        set.insert(1);
        set.insert(2);
        let drained: Vec<_> = set.drain(..).collect();
        
        assert_eq!(drained, vec![1, 2]);
        assert!(set.is_empty());
    }

    #[test]
    fn test_drain_partial_set() {
        let mut set = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        let drained: Vec<_> = set.drain(1..3).collect(); // Drench index 1 and 2
        
        assert_eq!(drained, vec![2, 3]);
        assert!(set.contains(&1));
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_drain_empty_set() {
        let mut set: IndexSet<u32> = IndexSet::new();
        let drained: Vec<_> = set.drain(..).collect();
        
        assert!(drained.is_empty());
        assert!(set.is_empty());
    }
}

#[cfg(test)]
mod tests_llm_16_617 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_first_non_empty() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        assert_eq!(set.first(), Some(&1));
    }

    #[test]
    fn test_first_empty() {
        let set: IndexSet<i32> = IndexSet::new();
        assert_eq!(set.first(), None);
    }

    #[test]
    fn test_first_with_duplicates() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(1);

        assert_eq!(set.first(), Some(&1));
    }
    
    #[test]
    fn test_first_after_removal() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.remove(&1);

        assert_eq!(set.first(), Some(&2));
    }
}

#[cfg(test)]
mod tests_llm_16_619 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_get_full_existing_key() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(10);
        set.insert(20);
        set.insert(30);
        
        let result = set.get_full(&20);
        assert_eq!(result, Some((1, &20)));
    }

    #[test]
    fn test_get_full_non_existing_key() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(10);
        set.insert(20);
        
        let result = set.get_full(&30);
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_full_empty_set() {
        let set: IndexSet<i32> = IndexSet::new();
        
        let result = set.get_full(&10);
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_full_multiple_insertions() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(1); // Duplicate insertion
        
        let result = set.get_full(&1);
        assert_eq!(result, Some((0, &1)));
        
        let result = set.get_full(&2);
        assert_eq!(result, Some((1, &2)));
    }
}

#[cfg(test)]
mod tests_llm_16_620 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_get_index_with_valid_index() {
        let mut set = IndexSet::new();
        set.insert("a");
        set.insert("b");
        set.insert("c");
        
        assert_eq!(set.get_index(0), Some(&"a"));
        assert_eq!(set.get_index(1), Some(&"b"));
        assert_eq!(set.get_index(2), Some(&"c"));
    }

    #[test]
    fn test_get_index_with_invalid_index() {
        let set = IndexSet::<&str>::new();
        
        assert_eq!(set.get_index(0), None);
        
        let mut set = IndexSet::new();
        set.insert("a");
        set.insert("b");
        
        assert_eq!(set.get_index(2), None);
    }

    #[test]
    fn test_get_index_boundary_cases() {
        let mut set = IndexSet::new();
        set.insert("first");
        assert_eq!(set.get_index(0), Some(&"first"));
        
        set.insert("second");
        assert_eq!(set.get_index(1), Some(&"second"));
        
        assert_eq!(set.get_index(2), None);
    }

    #[test]
    fn test_get_index_empty_set() {
        let set: IndexSet<&str> = IndexSet::new();
        assert_eq!(set.get_index(0), None);
    }
}

#[cfg(test)]
mod tests_llm_16_625 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_insert_before() {
        let mut set: IndexSet<char> = IndexSet::new();
        set.extend('a'..='z');

        // Test inserting a new value '*'
        assert_eq!(set.get_index_of(&'*'), None);
        assert_eq!(set.insert_before(10, '*'), (10, true));
        assert_eq!(set.get_index_of(&'*'), Some(10));

        // Test moving an existing value 'a' up
        assert_eq!(set.insert_before(10, 'a'), (9, false));
        assert_eq!(set.get_index_of(&'a'), Some(9));
        assert_eq!(set.get_index_of(&'*'), Some(10));

        // Test moving an existing value 'z' down
        assert_eq!(set.insert_before(10, 'z'), (10, false));
        assert_eq!(set.get_index_of(&'z'), Some(10));
        assert_eq!(set.get_index_of(&'*'), Some(11));

        // Test inserting before the endpoint
        assert_eq!(set.insert_before(set.len(), '*'), (26, false));
        assert_eq!(set.get_index_of(&'*'), Some(26));
        assert_eq!(set.insert_before(set.len(), '+'), (27, true));
        assert_eq!(set.get_index_of(&'+'), Some(27));
        assert_eq!(set.len(), 28);
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 26 but the index is 27. Expected index <= len")]
    fn test_insert_before_out_of_bounds() {
        let mut set: IndexSet<char> = IndexSet::new();
        set.extend('a'..='z');
        // This should panic since index is out of bounds
        let _ = set.insert_before(27, 'x');
    }
}

#[cfg(test)]
mod tests_llm_16_627 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_insert_sorted() {
        let mut set = IndexSet::new();
        assert_eq!(set.insert_sorted(5), (0, true)); // Inserting first element
        assert_eq!(set.insert_sorted(3), (0, true)); // Inserting before
        assert_eq!(set.insert_sorted(4), (1, true)); // Inserting in between
        assert_eq!(set.insert_sorted(5), (2, false)); // Inserting duplicate
        assert_eq!(set.insert_sorted(2), (0, true)); // Inserting before all
        assert_eq!(set.insert_sorted(6), (5, true)); // Inserting at the end
    }

    #[test]
    fn test_insert_sorted_with_order() {
        let mut set = IndexSet::new();
        assert_eq!(set.insert_sorted(10), (0, true)); // Inserting 10
        assert_eq!(set.insert_sorted(20), (1, true)); // Inserting 20
        assert_eq!(set.insert_sorted(15), (1, true)); // Inserting 15 in between
        assert_eq!(set.insert_sorted(20), (3, false)); // Inserting duplicate 20
        assert_eq!(set.insert_sorted(5), (0, true)); // Inserting 5 at start
        assert_eq!(set.insert_sorted(25), (5, true)); // Inserting at end
    }

    #[test]
    fn test_insert_sorted_empty() {
        let mut set: IndexSet<i32> = IndexSet::new();
        assert_eq!(set.insert_sorted(1), (0, true)); // First element in empty set
    }
}

#[cfg(test)]
mod tests_llm_16_629 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_into_boxed_slice() {
        let mut index_set: IndexSet<i32, RandomState> = IndexSet::new();
        index_set.insert(1);
        index_set.insert(2);
        index_set.insert(3);
        
        let boxed_slice = index_set.into_boxed_slice();

        assert_eq!(boxed_slice.len(), 3);
        assert_eq!(boxed_slice[0], 1);
        assert_eq!(boxed_slice[1], 2);
        assert_eq!(boxed_slice[2], 3);
    }

    #[test]
    fn test_into_boxed_slice_empty() {
        let index_set: IndexSet<i32, RandomState> = IndexSet::new();
        let boxed_slice = index_set.into_boxed_slice();

        assert_eq!(boxed_slice.len(), 0);
    }
}

#[cfg(test)]
mod tests_llm_16_630 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_is_disjoint_with_no_common_elements() {
        let set1: IndexSet<i32> = (1..=3).collect();
        let set2: IndexSet<i32> = (4..=6).collect();
        assert!(set1.is_disjoint(&set2));
    }

    #[test]
    fn test_is_disjoint_with_common_elements() {
        let set1: IndexSet<i32> = (1..=3).collect();
        let set2: IndexSet<i32> = (3..=5).collect();
        assert!(!set1.is_disjoint(&set2));
    }

    #[test]
    fn test_is_disjoint_with_empty_set() {
        let set1: IndexSet<i32> = (1..=3).collect();
        let set2: IndexSet<i32> = IndexSet::new();
        assert!(set1.is_disjoint(&set2));
    }

    #[test]
    fn test_is_disjoint_with_identical_sets() {
        let set1: IndexSet<i32> = (1..=3).collect();
        let set2 = set1.clone();
        assert!(!set1.is_disjoint(&set2));
    }

    #[test]
    fn test_is_disjoint_larger_sets() {
        let set1: IndexSet<i32> = (1..=5).collect();
        let set2: IndexSet<i32> = (6..=10).collect();
        assert!(set1.is_disjoint(&set2));

        let set3: IndexSet<i32> = (4..=8).collect();
        assert!(!set1.is_disjoint(&set3));
    }
}

#[cfg(test)]
mod tests_llm_16_631 {
    use super::*; // Assuming your module imports the necessary items

use crate::*;
    use crate::IndexMap; // Assuming `IndexMap` is necessary for the tests

    #[test]
    fn test_is_empty() {
        let set: IndexSet<i32> = IndexSet::new();
        assert!(set.is_empty());

        let mut set_with_elements = IndexSet::new();
        set_with_elements.insert(1);
        assert!(!set_with_elements.is_empty());

        set_with_elements.remove(&1);
        assert!(set_with_elements.is_empty());
    }
}

#[cfg(test)]
mod tests_llm_16_632 {
    use super::*;

use crate::*;
    use crate::IndexSet;
    use std::collections::hash_map::RandomState;

    #[test]
    fn test_is_subset() {
        let set1: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![1, 2, 3]);
        let set2: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![1, 2, 3, 4, 5]);
        let set3: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![4, 5, 6]);

        assert!(set1.is_subset(&set2));
        assert!(!set1.is_subset(&set3));
        assert!(set1.is_subset(&set1)); // A set is always a subset of itself
        assert!(!set3.is_subset(&set1)); // Distinct elements
    }

    #[test]
    fn test_is_subset_empty() {
        let set1: IndexSet<i32, RandomState> = IndexSet::new();
        let set2: IndexSet<i32, RandomState> = IndexSet::new();
        let set3: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![1, 2, 3]);

        assert!(set1.is_subset(&set2)); // Both empty
        assert!(!set3.is_subset(&set1)); // Non-empty is not subset of empty
    }

    #[test]
    fn test_is_subset_same_elements_different_order() {
        let set1: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![3, 2, 1]);
        let set2: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![1, 2, 3]);

        assert!(set1.is_subset(&set2));
        assert!(set2.is_subset(&set1));
    }
}

#[cfg(test)]
mod tests_llm_16_633 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_is_superset() {
        let mut set_a: IndexSet<i32> = IndexSet::new();
        set_a.insert(1);
        set_a.insert(2);
        set_a.insert(3);

        let mut set_b: IndexSet<i32> = IndexSet::new();
        set_b.insert(1);
        set_b.insert(2);

        let mut set_c: IndexSet<i32> = IndexSet::new();
        set_c.insert(1);
        set_c.insert(4);

        assert!(set_a.is_superset(&set_b));
        assert!(!set_a.is_superset(&set_c));
        assert!(set_b.is_superset(&IndexSet::new())); // subset of empty
        assert!(!set_c.is_superset(&set_a)); // set_c doesn't contain all of set_a
    }

    #[test]
    fn test_is_superset_empty() {
        let set_a: IndexSet<i32> = IndexSet::new();
        let set_b: IndexSet<i32> = IndexSet::new();
        let set_c: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3]);

        assert!(set_a.is_superset(&set_b)); // empty set is superset of empty
        assert!(!set_a.is_superset(&set_c)); // empty set is not superset of non-empty
    }
}

#[cfg(test)]
mod tests_llm_16_634 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_iter() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_empty_iter() {
        let set: IndexSet<i32> = IndexSet::new();
        let mut iter = set.iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_ordered_iter() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(3);
        set.insert(1);
        set.insert(2);

        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_635 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_last_empty() {
        let set: IndexSet<i32> = IndexSet::new();
        assert_eq!(set.last(), None);
    }

    #[test]
    fn test_last_one_element() {
        let mut set = IndexSet::new();
        set.insert(1);
        assert_eq!(set.last(), Some(&1));
    }

    #[test]
    fn test_last_multiple_elements() {
        let mut set = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        assert_eq!(set.last(), Some(&3));
    }

    #[test]
    fn test_last_elements_after_removal() {
        let mut set = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.remove(&2);
        assert_eq!(set.last(), Some(&3));
    }

    #[test]
    fn test_last_after_removal_of_last() {
        let mut set = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.remove(&3);
        assert_eq!(set.last(), Some(&2));
    }

    #[test]
    fn test_last_removing_all_elements() {
        let mut set = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.remove(&1);
        set.remove(&2);
        assert_eq!(set.last(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_636 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_len() {
        let set: IndexSet<i32> = IndexSet::new();
        assert_eq!(set.len(), 0);

        let mut set = IndexSet::new();
        set.insert(1);
        set.insert(2);
        assert_eq!(set.len(), 2);

        set.remove(&1);
        assert_eq!(set.len(), 1);

        set.clear();
        assert_eq!(set.len(), 0);
    }
}

#[cfg(test)]
mod tests_llm_16_637 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
    fn test_move_index_out_of_bounds() {
        let mut set: IndexSet<u32> = IndexSet::new();
        set.move_index(0, 1);
    }

    #[test]
    fn test_move_index() {
        let mut set: IndexSet<u32> = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        // Moving an element up
        set.move_index(2, 0);
        assert_eq!(set.iter().copied().collect::<Vec<_>>(), vec![3, 1, 2]);

        // Moving an element down
        set.move_index(0, 2);
        assert_eq!(set.iter().copied().collect::<Vec<_>>(), vec![1, 2, 3]);
    }

    #[test]
    fn test_move_index_shift_up() {
        let mut set: IndexSet<u32> = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        // Move 2 to the start, expect shift up
        set.move_index(1, 0);
        assert_eq!(set.iter().copied().collect::<Vec<_>>(), vec![2, 1, 3]);
    }

    #[test]
    fn test_move_index_shift_down() {
        let mut set: IndexSet<u32> = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        // Move 2 to the end, expect shift down
        set.move_index(0, 2);
        assert_eq!(set.iter().copied().collect::<Vec<_>>(), vec![1, 3, 2]);
    }
}

#[cfg(test)]
mod tests_llm_16_638 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_partition_point() {
        let set: IndexSet<i32> = (0..10).collect();

        // Test partition point with a predicate that is true for values less than 5
        let index = set.partition_point(|&x| x < 5);
        assert_eq!(index, 5, "Partition point should be at index 5");

        // Test partition point with a predicate that is true for values less than 0
        let index = set.partition_point(|&x| x < 0);
        assert_eq!(index, 0, "Partition point should be at index 0");

        // Test partition point with a predicate that is true for values less than 10
        let index = set.partition_point(|&x| x < 10);
        assert_eq!(index, 10, "Partition point should be at index 10");

        // Test partition point with a predicate that is true for all values
        let index = set.partition_point(|_| true);
        assert_eq!(index, 10, "Partition point should be at index 10");

        // Test partition point with a predicate that is false for all values
        let index = set.partition_point(|_| false);
        assert_eq!(index, 0, "Partition point should be at index 0");
    }
}

#[cfg(test)]
mod tests_llm_16_640 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_remove_existing_key() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        assert!(set.remove(&2));
        assert!(!set.contains(&2));
        assert!(set.contains(&1));
        assert!(set.contains(&3));
    }

    #[test]
    fn test_remove_non_existing_key() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(1);
        set.insert(2);

        assert!(!set.remove(&3));
        assert!(set.contains(&1));
        assert!(set.contains(&2));
    }

    #[test]
    fn test_remove_key_from_empty_set() {
        let mut set: IndexSet<i32> = IndexSet::new();
        assert!(!set.remove(&1));
    }

    #[test]
    #[deprecated(note = "use swap_remove or shift_remove instead")]
    fn test_remove_deprecated() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        assert!(set.remove(&1));
        assert!(!set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
    }
}

#[cfg(test)]
mod tests_llm_16_642 {
    use super::*;

use crate::*;
    use crate::IndexSet; // Adjust the import path as necessary

    #[test]
    fn test_replace_full_new_value() {
        let mut set: IndexSet<i32> = IndexSet::new();
        let (index, replaced) = set.replace_full(42);
        assert_eq!(index, 0);
        assert_eq!(replaced, None);
        assert_eq!(set.len(), 1);
        assert!(set.contains(&42));
    }

    #[test]
    fn test_replace_full_existing_value() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(42);
        let (index, replaced) = set.replace_full(42);
        assert_eq!(index, 0);
        assert_eq!(replaced, Some(42));
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_replace_full_multiple_values() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(10);
        set.insert(20);
        let (index, replaced) = set.replace_full(10);
        assert_eq!(index, 0);
        assert_eq!(replaced, Some(10));
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_replace_full_ordering() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(10);
        set.insert(20);
        let (index1, _) = set.replace_full(10);
        let (index2, _) = set.replace_full(30);
        assert_eq!(index1, 0);
        assert_eq!(index2, 2);
        assert_eq!(set.len(), 3);
        assert_eq!(set.get_index(0), Some(&10));
        assert_eq!(set.get_index(1), Some(&20));
        assert_eq!(set.get_index(2), Some(&30));
    }
}

#[cfg(test)]
mod tests_llm_16_643 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_reserve() {
        let mut index_set: IndexSet<u32, _> = IndexSet::new();
        index_set.reserve(10);
        assert!(index_set.capacity() >= 10);
        
        index_set.reserve(20);
        assert!(index_set.capacity() >= 30);
        
        index_set.insert(1);
        index_set.insert(2);
        index_set.insert(3);
        index_set.reserve(5);
        assert!(index_set.capacity() >= 35);
    }

    #[test]
    fn test_reserve_zero() {
        let mut index_set: IndexSet<u32, _> = IndexSet::new();
        let initial_capacity = index_set.capacity();
        index_set.reserve(0);
        assert_eq!(index_set.capacity(), initial_capacity);
    }
}

#[cfg(test)]
mod tests_llm_16_644 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_reserve_exact() {
        let mut index_set: IndexSet<u32> = IndexSet::new();
        let initial_capacity = index_set.capacity();
        
        // Reserve exact 5 more
        index_set.reserve_exact(5);
        assert!(index_set.capacity() >= initial_capacity + 5);
        
        // Reserve exact 0
        index_set.reserve_exact(0);
        assert_eq!(index_set.capacity(), initial_capacity + 5);
        
        // Add some elements
        index_set.insert(1);
        index_set.insert(2);

        // Reserve exact for more elements
        let new_capacity = index_set.capacity();
        index_set.reserve_exact(10);
        assert!(index_set.capacity() >= new_capacity + 10);
    }

    #[test]
    fn test_reserve_exact_does_not_over_allocate() {
        let mut index_set: IndexSet<u32> = IndexSet::with_capacity(3);
        index_set.reserve_exact(2);
        let initial_capacity = index_set.capacity();
        
        index_set.reserve_exact(3);
        assert_eq!(index_set.capacity(), initial_capacity);
    }

    #[test]
    fn test_reserve_exact_after_inserts() {
        let mut index_set: IndexSet<u32> = IndexSet::new();
        index_set.insert(1);
        index_set.insert(2);
        
        let initial_capacity = index_set.capacity();
        index_set.reserve_exact(5);
        
        assert!(index_set.capacity() >= initial_capacity);
        assert!(index_set.capacity() >= initial_capacity + 5);
    }

    #[test]
    fn test_reserve_exact_no_capacity_increase() {
        let mut index_set: IndexSet<u32> = IndexSet::with_capacity(0);
        index_set.reserve_exact(10);
        
        assert!(index_set.capacity() >= 10);
    }
}

#[cfg(test)]
mod tests_llm_16_645 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_retain_some_elements() {
        let mut set = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        
        set.retain(|&x| x > 1); // Retain only elements greater than 1
        
        let expected: IndexSet<_> = vec![2, 3].into_iter().collect();
        assert_eq!(set, expected);
    }

    #[test]
    fn test_retain_no_elements() {
        let mut set = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        
        set.retain(|&x| x > 3); // Retain only elements greater than 3, which will remove all
        
        let expected: IndexSet<_> = IndexSet::new();
        assert_eq!(set, expected);
    }

    #[test]
    fn test_retain_all_elements() {
        let mut set = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        
        set.retain(|&x| x > 0); // Retain all elements
        
        let expected: IndexSet<_> = vec![1, 2, 3].into_iter().collect();
        assert_eq!(set, expected);
    }

    #[test]
    fn test_retain_with_no_changes() {
        let mut set = IndexSet::new();
        set.insert(1);
        set.insert(2);
        
        set.retain(|&x| x < 3); // No elements removed
        
        let expected: IndexSet<_> = vec![1, 2].into_iter().collect();
        assert_eq!(set, expected);
    }
}

#[cfg(test)]
mod tests_llm_16_647 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_shift_insert_insert_new_value() {
        let mut set: IndexSet<char> = ('a'..='z').collect();
        assert_eq!(set.shift_insert(10, '*'), true);
        assert_eq!(set.get_index_of(&'*'), Some(10));
    }

    #[test]
    fn test_shift_insert_move_existing_value() {
        let mut set: IndexSet<char> = ('a'..='z').collect();
        assert_eq!(set.shift_insert(10, 'a'), false);
        assert_eq!(set.get_index_of(&'a'), Some(10));
    }

    #[test]
    fn test_shift_insert_move_existing_value_shift_down() {
        let mut set: IndexSet<char> = ('a'..='z').collect();
        assert_eq!(set.shift_insert(9, 'z'), false);
        assert_eq!(set.get_index_of(&'z'), Some(9));
    }

    #[test]
    fn test_shift_insert_move_existing_value_at_last_index() {
        let mut set: IndexSet<char> = ('a'..='z').collect();
        assert_eq!(set.len(), 27);
        assert_eq!(set.shift_insert(set.len() - 1, '*'), false); // moving existing value
        assert_eq!(set.get_index_of(&'*'), Some(26));
    }

    #[test]
    fn test_shift_insert_insert_at_end() {
        let mut set: IndexSet<char> = ('a'..='z').collect();
        assert_eq!(set.shift_insert(set.len(), '+'), true);
        assert_eq!(set.get_index_of(&'+'), Some(27));
        assert_eq!(set.len(), 28);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_shift_insert_panics_on_move_existing_value() {
        let mut set: IndexSet<char> = ('a'..='z').collect();
        set.shift_insert(set.len(), 'a');
    }
}

#[cfg(test)]
mod tests_llm_16_648 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_shift_remove_present() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        assert!(set.shift_remove(&2));
        assert!(!set.contains(&2));
        assert_eq!(set.len(), 2);
        assert_eq!(set.iter().collect::<Vec<_>>(), vec![&1, &3]);
    }

    #[test]
    fn test_shift_remove_absent() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(1);
        set.insert(2);

        assert!(!set.shift_remove(&3)); // 3 is not in the set
        assert_eq!(set.len(), 2);
        assert_eq!(set.iter().collect::<Vec<_>>(), vec![&1, &2]);
    }

    #[test]
    fn test_shift_remove_first() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        assert!(set.shift_remove(&1));
        assert_eq!(set.len(), 2);
        assert_eq!(set.iter().collect::<Vec<_>>(), vec![&2, &3]);
    }

    #[test]
    fn test_shift_remove_last() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        assert!(set.shift_remove(&3));
        assert_eq!(set.len(), 2);
        assert_eq!(set.iter().collect::<Vec<_>>(), vec![&1, &2]);
    }

    #[test]
    fn test_shift_remove_empty() {
        let mut set: IndexSet<i32> = IndexSet::new();
        assert!(!set.shift_remove(&1)); // 1 is not in the set
        assert_eq!(set.len(), 0);
    }
}

#[cfg(test)]
mod tests_llm_16_649 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_shift_remove_full() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(10);
        set.insert(20);
        set.insert(30);

        // Test removing existing element
        let result = set.shift_remove_full(&20);
        assert_eq!(result, Some((1, 20))); // index 1, value 20
        assert_eq!(set.len(), 2);
        assert!(set.contains(&10));
        assert!(set.contains(&30));
        assert!(!set.contains(&20));

        // Test removing first element
        let result = set.shift_remove_full(&10);
        assert_eq!(result, Some((0, 10))); // index 0, value 10
        assert_eq!(set.len(), 1);
        assert!(set.contains(&30));
        assert!(!set.contains(&10));

        // Test removing last element
        let result = set.shift_remove_full(&30);
        assert_eq!(result, Some((0, 30))); // index 0, value 30
        assert_eq!(set.len(), 0);
        assert!(!set.contains(&30));

        // Test removing non-existent element
        let result = set.shift_remove_full(&40);
        assert_eq!(result, None);
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_shift_remove_full_empty() {
        let mut set: IndexSet<i32> = IndexSet::new();
        // Test removing from an empty set
        let result = set.shift_remove_full(&1);
        assert_eq!(result, None);
    }
}

#[cfg(test)]
mod tests_llm_16_650 {
    use super::*; // Make sure to bring the relevant items into scope

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_shift_remove_index() {
        let mut index_set: IndexSet<u32> = IndexSet::new();
        index_set.insert(1);
        index_set.insert(2);
        index_set.insert(3);

        // Remove the item at index 1
        let removed_value = index_set.shift_remove_index(1);
        assert_eq!(removed_value, Some(2)); // Should remove the value 2

        // Check the remaining items
        let remaining: Vec<_> = index_set.iter().collect();
        assert_eq!(remaining, vec![&1, &3]); // Should only have 1 and 3 left

        // Check shifting
        let removed_value_again = index_set.shift_remove_index(0);
        assert_eq!(removed_value_again, Some(1)); // Should remove the value 1

        let remaining_after_removal: Vec<_> = index_set.iter().collect();
        assert_eq!(remaining_after_removal, vec![&3]); // Should only have 3 left

        // Check out of bounds
        let out_of_bounds = index_set.shift_remove_index(1);
        assert_eq!(out_of_bounds, None); // Should return None since index 1 is out of bounds
    }

    #[test]
    fn test_shift_remove_index_empty() {
        let mut index_set: IndexSet<u32> = IndexSet::new();
        // Check remove on empty index set
        let removed_value = index_set.shift_remove_index(0);
        assert_eq!(removed_value, None); // Should return None, as there are no items
    }

    #[test]
    fn test_shift_remove_index_single_item() {
        let mut index_set: IndexSet<u32> = IndexSet::new();
        index_set.insert(42);

        // Remove the only item
        let removed_value = index_set.shift_remove_index(0);
        assert_eq!(removed_value, Some(42)); // Should remove the value 42

        // Check it is empty now
        assert!(index_set.is_empty());
    }

    #[test]
    fn test_shift_remove_index_multiple_removals() {
        let mut index_set: IndexSet<u32> = IndexSet::new();
        index_set.insert(10);
        index_set.insert(20);
        index_set.insert(30);
        index_set.insert(40);

        // Remove the first item
        assert_eq!(index_set.shift_remove_index(0), Some(10));
        assert_eq!(index_set.len(), 3);
        
        // Remove the now first item
        assert_eq!(index_set.shift_remove_index(0), Some(20));
        assert_eq!(index_set.len(), 2);
        
        // Remove the last item
        assert_eq!(index_set.shift_remove_index(1), Some(40));
        assert_eq!(index_set.len(), 1);
        
        // Finally, remove the last remaining item
        assert_eq!(index_set.shift_remove_index(0), Some(30));
        assert!(index_set.is_empty());
    }
}

#[cfg(test)]
mod tests_llm_16_651 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_shift_take() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        assert_eq!(set.shift_take(&2), Some(2));
        assert_eq!(set.contains(&2), false);
        assert_eq!(set.shift_take(&4), None);
        assert_eq!(set.shift_take(&3), Some(3));
        assert_eq!(set.shift_take(&1), Some(1));
        assert_eq!(set.shift_take(&1), None);
    }

    #[test]
    fn test_shift_take_empty_set() {
        let mut set: IndexSet<i32> = IndexSet::new();
        assert_eq!(set.shift_take(&1), None);
    }

    #[test]
    fn test_shift_take_multiple_elements() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);

        assert_eq!(set.shift_take(&3), Some(3));
        assert_eq!(set.shift_take(&1), Some(1));
        assert_eq!(set.shift_take(&4), Some(4));
        assert_eq!(set.shift_take(&2), Some(2));
    }
}

#[cfg(test)]
mod tests_llm_16_652 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_shrink_to() {
        let mut set = IndexSet::with_capacity(10);
        
        // Adding elements
        set.insert("a");
        set.insert("b");
        set.insert("c");
        
        // Initial capacity
        let initial_capacity = set.capacity();
        
        // Shrinking to a lower number
        set.shrink_to(2);
        
        // Capacity should not shrink below the number of elements
        assert!(set.capacity() >= 2);
        assert!(set.capacity() <= initial_capacity);

        // Shrink to 1
        set.shrink_to(1);
        
        // Capacity should not shrink below the number of elements
        assert!(set.capacity() >= 1);
    }

    #[test]
    fn test_shrink_to_empty() {
        let mut set: IndexSet<&str> = IndexSet::new();
        
        // Initial capacity should be 0
        assert_eq!(set.capacity(), 0);
        
        // Shrink to 1 should not change capacity as the set is empty
        set.shrink_to(1);
        assert_eq!(set.capacity(), 0);
        
        // Insert an element
        set.insert("a");
        
        // Shrink again, capacity should be at least 1
        set.shrink_to(1);
        assert!(set.capacity() >= 1);
    }

    #[test]
    fn test_shrink_to_greater_capacity() {
        let mut set = IndexSet::with_capacity(5);
        set.insert("a");
        set.insert("b");
        
        // Shrink to a capacity greater than current elements
        set.shrink_to(10);
        
        // Capacity should be at least as large as the size
        assert!(set.capacity() >= set.len());
    }
}

#[cfg(test)]
mod tests_llm_16_653 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_shrink_to_fit() {
        let mut set: IndexSet<i32> = IndexSet::new();

        // Initially, capacity should be 0
        assert_eq!(set.capacity(), 0);

        // Insert elements to increase capacity
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);

        // Verify capacity is increased
        assert!(set.capacity() > 0);

        // Clear the set, which should leave capacity
        set.clear();
        assert!(set.capacity() > 0);

        // Now shrink to fit
        set.shrink_to_fit();

        // Capacity should be equal to the length (which is now 0)
        assert_eq!(set.capacity(), 0);
    }

    #[test]
    fn test_shrink_to_fit_non_empty() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(1);
        set.insert(2);

        // Verify capacity
        assert!(set.capacity() > 0);

        // Shrink to fit
        set.shrink_to_fit();

        // Capacity should still be > 0 as we have elements
        assert!(set.capacity() > 0);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_shrink_to_fit_empty_set() {
        let mut set: IndexSet<i32> = IndexSet::new();

        // Initially shrink_to_fit should have no effect
        set.shrink_to_fit();
        assert_eq!(set.capacity(), 0);
    }
}

#[cfg(test)]
mod tests_llm_16_654 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_sort() {
        let mut set: IndexSet<i32> = IndexSet::from_iter(vec![3, 1, 4, 1, 5, 9, 2, 6, 5]);
        set.sort();
        let sorted: Vec<i32> = set.iter().cloned().collect();
        assert_eq!(sorted, vec![1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_sort_empty_set() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.sort();
        let sorted: Vec<i32> = set.iter().cloned().collect();
        assert_eq!(sorted, vec![]);
    }

    #[test]
    fn test_sort_single_element() {
        let mut set: IndexSet<i32> = IndexSet::from_iter(vec![42]);
        set.sort();
        let sorted: Vec<i32> = set.iter().cloned().collect();
        assert_eq!(sorted, vec![42]);
    }

    #[test]
    fn test_sort_stable() {
        let mut set: IndexSet<&str> = IndexSet::from_iter(vec!["apple", "banana", "apple", "cherry"]);
        set.sort();
        let sorted: Vec<&str> = set.iter().cloned().collect();
        assert_eq!(sorted, vec!["apple", "apple", "banana", "cherry"]);
    }
}

#[cfg(test)]
mod tests_llm_16_656 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_sort_by_cached_key() {
        let mut set = IndexSet::from([3, 1, 2, 0]);
        set.sort_by_cached_key(|&key| key);
        let sorted: Vec<_> = set.iter().cloned().collect();
        assert_eq!(sorted, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_sort_by_cached_key_stable() {
        let mut set = IndexSet::from([4, 1, 3, 4, 2, 1]);
        set.sort_by_cached_key(|&key| key);
        let sorted: Vec<_> = set.iter().cloned().collect();
        assert_eq!(sorted, vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn test_sort_by_cached_key_empty() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.sort_by_cached_key(|&key| key);
        let sorted: Vec<_> = set.iter().cloned().collect();
        assert_eq!(sorted, Vec::<i32>::new());
    }

    #[test]
    fn test_sort_by_cached_key_custom_key() {
        let mut set = IndexSet::from([("apple", 2), ("banana", 1), ("cherry", 3)]);
        set.sort_by_cached_key(|&(key, _)| key);
        let sorted: Vec<_> = set.iter().cloned().collect();
        assert_eq!(sorted, vec![("apple", 2), ("banana", 1), ("cherry", 3)]);
    }

    #[test]
    fn test_sort_by_cached_key_complex() {
        let mut set = IndexSet::from([("dog", 2), ("cat", 3), ("bird", 1)]);
        set.sort_by_cached_key(|&(key, _)| key.len());
        let sorted: Vec<_> = set.iter().cloned().collect();
        assert_eq!(sorted, vec![("dog", 2), ("cat", 3), ("bird", 1)]);
    }
}

#[cfg(test)]
mod tests_llm_16_659 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_sorted_by() {
        let mut set = IndexSet::new();
        set.insert(3);
        set.insert(1);
        set.insert(2);
        
        let sorted_iter = set.sorted_by(|a, b| a.cmp(b));
        let sorted_vec: Vec<_> = sorted_iter.collect();

        assert_eq!(sorted_vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_sorted_by_descending() {
        let mut set = IndexSet::new();
        set.insert(1);
        set.insert(3);
        set.insert(2);

        let sorted_iter = set.sorted_by(|a, b| b.cmp(a));
        let sorted_vec: Vec<_> = sorted_iter.collect();

        assert_eq!(sorted_vec, vec![3, 2, 1]);
    }

    #[test]
    fn test_sorted_by_stability() {
        let mut set = IndexSet::new();
        set.insert(1);
        set.insert(3);
        set.insert(2);
        set.insert(2); // Duplicate to test stability

        let sorted_iter = set.sorted_by(|a, b| a.cmp(b));
        let sorted_vec: Vec<_> = sorted_iter.collect();

        assert_eq!(sorted_vec, vec![2, 2, 1, 3]); // Duplicate 2 should remain in order
    }
}

#[cfg(test)]
mod tests_llm_16_660 {
    use super::*;

use crate::*;
    use crate::IndexMap;
    use std::cmp::Ordering;

    #[test]
    fn test_sorted_unstable_by() {
        let mut index_set = IndexSet::new();
        index_set.insert(3);
        index_set.insert(1);
        index_set.insert(4);
        index_set.insert(2);

        let sorted: Vec<_> = index_set.sorted_unstable_by(|a, b| a.cmp(b)).collect();
        
        assert_eq!(sorted, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_sorted_unstable_by_reverse() {
        let mut index_set = IndexSet::new();
        index_set.insert(4);
        index_set.insert(3);
        index_set.insert(2);
        index_set.insert(1);

        let sorted: Vec<_> = index_set.sorted_unstable_by(|a, b| b.cmp(a)).collect();
        
        assert_eq!(sorted, vec![4, 3, 2, 1]);
    }

    #[test]
    fn test_sorted_unstable_by_with_equal_elements() {
        let mut index_set = IndexSet::new();
        index_set.insert(1);
        index_set.insert(2);
        index_set.insert(1); // Duplicate

        let sorted: Vec<_> = index_set.sorted_unstable_by(|a, b| a.cmp(b)).collect();
        
        assert_eq!(sorted, vec![1, 1, 2]);
    }
}

#[cfg(test)]
mod tests_llm_16_661 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_splice_basic() {
        let mut set = IndexSet::from([0, 1, 2, 3, 4]);
        let new = [5, 4, 3, 2, 1];
        let removed: Vec<_> = set.splice(2..4, new).collect();
        
        assert!(set.into_iter().eq([0, 1, 5, 3, 2, 4]));
        assert_eq!(removed, &[2, 3]);
    }

    #[test]
    fn test_splice_empty() {
        let mut set: IndexSet<i32> = IndexSet::new();
        let new = [1, 2, 3];
        let removed: Vec<_> = set.splice(0..0, new).collect();

        assert!(set.is_empty());
        assert_eq!(removed, &[]);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_splice_panic_start_greater_than_end() {
        let mut set = IndexSet::from([0, 1, 2, 3]);
        set.splice(3..1, [4, 5]);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_splice_panic_end_greater_than_length() {
        let mut set = IndexSet::from([0, 1, 2]);
        set.splice(0..4, [3]);
    }

    #[test]
    fn test_splice_with_existing_elements() {
        let mut set = IndexSet::from([0, 1, 2, 3, 4]);
        let new = [2, 5, 6];
        let removed: Vec<_> = set.splice(1..3, new).collect();

        assert!(set.into_iter().eq([0, 2, 5, 6, 4]));
        assert_eq!(removed, &[1, 2]);
    }

    #[test]
    fn test_splice_no_elements_removed() {
        let mut set = IndexSet::from([1, 2, 3]);
        let new = [4];
        let removed: Vec<_> = set.splice(1..1, new).collect();

        assert!(set.into_iter().eq([1, 4, 2, 3]));
        assert_eq!(removed, &[]);
    }
}

#[cfg(test)]
mod tests_llm_16_662 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_split_off() {
        let mut index_set: IndexMap<i32, ()> = IndexMap::new();
        index_set.insert(1, ());
        index_set.insert(2, ());
        index_set.insert(3, ());

        let split_index = 2;
        let split_result = index_set.split_off(split_index);

        // Original should contain 1, 2
        assert_eq!(index_set.len(), 2);
        assert!(index_set.contains_key(&1));
        assert!(index_set.contains_key(&2));
        assert!(!index_set.contains_key(&3));

        // Split result should contain 3
        assert_eq!(split_result.len(), 1);
        assert!(split_result.contains_key(&3));
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_split_off_panic() {
        let mut index_set: IndexMap<i32, ()> = IndexMap::new();
        index_set.insert(1, ());
        index_set.insert(2, ());

        index_set.split_off(3); // Should panic since index 3 is out of bounds
    }

    #[test]
    fn test_split_off_empty() {
        let mut index_set: IndexMap<i32, ()> = IndexMap::new();
        let split_result = index_set.split_off(0);

        // Splitting an empty set should result in an empty set
        assert!(split_result.is_empty());
        assert!(index_set.is_empty());
    }

    #[test]
    fn test_split_off_first_index() {
        let mut index_set: IndexMap<i32, ()> = IndexMap::new();
        index_set.insert(1, ());
        index_set.insert(2, ());
        index_set.insert(3, ());

        let split_result = index_set.split_off(0);

        // Original should be empty
        assert!(index_set.is_empty());

        // Split result should contain all elements
        assert_eq!(split_result.len(), 3);
        assert!(split_result.contains_key(&1));
        assert!(split_result.contains_key(&2));
        assert!(split_result.contains_key(&3));
    }
}

#[cfg(test)]
mod tests_llm_16_664 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_swap_remove() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        // Test removing an existing element
        assert!(set.swap_remove(&2));
        assert!(!set.contains(&2));
        assert_eq!(set.len(), 2);

        // Test removing a non-existing element
        assert!(!set.swap_remove(&5));
        assert_eq!(set.len(), 2);

        // Test removing the last element
        assert!(set.swap_remove(&3));
        assert!(!set.contains(&3));
        assert_eq!(set.len(), 1);
        assert!(set.swap_remove(&1));
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_swap_remove_no_effect_on_last() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(1);
        set.insert(2);

        // Remove the last element
        assert!(set.swap_remove(&2));
        assert!(set.len() == 1);
        assert!(set.contains(&1));
    }

    #[test]
    fn test_swap_remove_empty() {
        let mut set: IndexSet<i32> = IndexSet::new();
        assert!(!set.swap_remove(&1)); // Removing from an empty set
        assert_eq!(set.len(), 0);
    }
}

#[cfg(test)]
mod tests_llm_16_665 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_swap_remove_full_existing_key() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        let result = set.swap_remove_full(&2);
        assert_eq!(result, Some((1, 2))); // Key 2 was at index 1
        assert!(!set.contains(&2)); // Key 2 should no longer be present
    }

    #[test]
    fn test_swap_remove_full_non_existing_key() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        let result = set.swap_remove_full(&4);
        assert_eq!(result, None); // Key 4 does not exist
    }

    #[test]
    fn test_swap_remove_full_edge_case() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(42);

        let result = set.swap_remove_full(&42);
        assert_eq!(result, Some((0, 42))); // Key 42 was at index 0
        assert!(set.is_empty()); // Set should be empty now
    }

    #[test]
    fn test_swap_remove_full_multiple_keys() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);
        set.insert(5);

        let result = set.swap_remove_full(&3);
        assert_eq!(result, Some((2, 3))); // Key 3 was at index 2
        assert_eq!(set.len(), 4); // Set should have 4 keys now
    }
}

#[cfg(test)]
mod tests_llm_16_666 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_swap_remove_index() {
        let mut set: IndexSet<i32> = IndexSet::new();
        
        set.insert(1);
        set.insert(2);
        set.insert(3);
        
        // Check initial length
        assert_eq!(set.len(), 3);
        
        // Swap remove index 1 (should remove 2)
        assert_eq!(set.swap_remove_index(1), Some(2));
        assert_eq!(set.len(), 2);
        assert!(!set.contains(&2));
        
        // Check the remaining elements
        let remaining: Vec<_> = set.iter().cloned().collect();
        assert!(remaining.contains(&1));
        assert!(remaining.contains(&3));
        
        // Swap remove index 0 (should remove 1)
        assert_eq!(set.swap_remove_index(0), Some(1));
        assert_eq!(set.len(), 1);
        assert!(!set.contains(&1));
        
        // Swap remove index 0 (should remove 3)
        assert_eq!(set.swap_remove_index(0), Some(3));
        assert_eq!(set.len(), 0);
        
        // Attempt to swap remove from an empty set
        assert_eq!(set.swap_remove_index(0), None);
    }

    #[test]
    fn test_swap_remove_index_out_of_bounds() {
        let mut set: IndexSet<i32> = IndexSet::new();
        
        set.insert(1);
        set.insert(2);
        
        // Check that index 2 is out of bounds
        assert_eq!(set.swap_remove_index(2), None);
    }
}

#[cfg(test)]
mod tests_llm_16_667 {
    use super::*;

use crate::*;
    use crate::{IndexMap, IndexSet};

    #[test]
    fn test_swap_take() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        assert_eq!(set.swap_take(&2), Some(2));
        assert_eq!(set.contains(&2), false);
        assert_eq!(set.len(), 2);
        assert_eq!(set.swap_take(&4), None); // 4 is not in the set
    }

    #[test]
    fn test_swap_take_removes_last_element() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(1);
        set.insert(2);

        assert_eq!(set.swap_take(&1), Some(1));
        assert_eq!(set.swap_take(&2), Some(2));
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_swap_take_no_elements() {
        let mut set: IndexSet<i32> = IndexSet::new();
        
        assert_eq!(set.swap_take(&1), None); // set is empty
    }
}

#[cfg(test)]
mod tests_llm_16_670 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_truncate() {
        let mut set = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        
        set.truncate(2);
        assert_eq!(set.len(), 2);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(!set.contains(&3));

        set.truncate(1);
        assert_eq!(set.len(), 1);
        assert!(set.contains(&1));
        assert!(!set.contains(&2));
        assert!(!set.contains(&3));

        set.truncate(3);
        assert_eq!(set.len(), 1);
    }
}

#[cfg(test)]
mod tests_llm_16_671 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn try_reserve_success() {
        let mut index_set: IndexSet<i32> = IndexSet::new();
        index_set.insert(1);
        index_set.insert(2);
        
        // Ensure we can reserve additional capacity
        assert!(index_set.try_reserve(5).is_ok());
        assert!(index_set.try_reserve(0).is_ok()); // Reserving zero should succeed
    }

    #[test]
    fn try_reserve_error() {
        // Assuming a specific implementation where reserving
        // more than capacity would return an error

        let mut index_set: IndexSet<i32> = IndexSet::with_capacity(2);
        index_set.insert(1);
        index_set.insert(2);
        
        // Intentionally reserve more than what can be accommodated
        let result = index_set.try_reserve(usize::MAX);
        assert!(result.is_err()); // Expect an error
    }

    #[test]
    fn try_reserve_on_empty() {
        let mut index_set: IndexSet<i32> = IndexSet::new();
        // Reserving when the index set is empty
        assert!(index_set.try_reserve(10).is_ok());
    }

    #[test]
    fn try_reserve_exact_success() {
        let mut index_set = IndexSet::new();
        index_set.insert(1);
        index_set.insert(2);

        // Ensure we can reserve exact additional capacity
        assert!(index_set.try_reserve_exact(5).is_ok());
    }

    #[test]
    fn try_reserve_exact_error() {
        // Assuming a specific implementation where reserving
        // more than capacity would return an error

        let mut index_set: IndexSet<i32> = IndexSet::with_capacity(2);
        index_set.insert(1);
        index_set.insert(2);
        
        // Intentionally reserve more than what can be accommodated
        let result = index_set.try_reserve_exact(usize::MAX);
        assert!(result.is_err()); // Expect an error
    }

    #[test]
    fn try_reserve_with_previous_elements() {
        let mut index_set = IndexSet::new();
        index_set.insert(1);
        index_set.insert(2);

        // Reserve additional elements
        assert!(index_set.try_reserve(5).is_ok());
        assert!(index_set.len() == 2); // Ensure elements remain intact
    }
}

#[cfg(test)]
mod tests_llm_16_672 {
    use super::*; // or use indexmap::set::IndexSet

use crate::*;
    use crate::IndexSet; // Adjust import according to your module path
    use std::collections::hash_map::RandomState;

    #[test]
    fn test_try_reserve_exact_success() {
        let mut set: IndexSet<u32, RandomState> = IndexSet::new();
        set.insert(1);
        set.insert(2);

        assert_eq!(set.len(), 2);
        assert!(set.try_reserve_exact(1).is_ok());
        assert!(set.try_reserve_exact(2).is_ok());
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_try_reserve_exact_failure() {
        let mut set: IndexSet<u32, RandomState> = IndexSet::with_capacity(1);
        set.insert(1);

        assert!(set.try_reserve_exact(2).is_ok());
        assert_eq!(set.len(), 1); // Ensure length does not change until a successful `insert`
    }

    #[test]
    fn test_try_reserve_exact_edge_case() {
        let mut set: IndexSet<u32, RandomState> = IndexSet::new();
        set.insert(1);
        set.insert(2);

        // Initially have 2 elements, try to reserve 0
        assert!(set.try_reserve_exact(0).is_ok());
        assert_eq!(set.len(), 2);

        // Attempt to reserve memory above current capacity
        assert!(set.try_reserve_exact(10).is_ok());
    }
}

#[cfg(test)]
mod tests_llm_16_674 {
    use super::*;

use crate::*;
    use crate::IndexSet;
    use std::collections::hash_map::RandomState;

    #[test]
    fn test_with_capacity_and_hasher() {
        let set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(10, RandomState::new());
        assert_eq!(set.len(), 0);
        assert!(set.is_empty());
    }

    #[test]
    fn test_with_capacity_and_hasher_zero() {
        let set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());
        assert_eq!(set.len(), 0);
        assert!(set.is_empty());
    }

    #[test]
    fn test_with_capacity_and_hasher_non_zero_capacity() {
        let mut set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(5, RandomState::new());
        assert_eq!(set.len(), 0);
        assert!(set.is_empty());

        set.insert(1);
        set.insert(2);
        assert_eq!(set.len(), 2);
        assert!(!set.is_empty());
    }
}

#[cfg(test)]
mod tests_llm_16_675 {
    use super::*;

use crate::*;
    use crate::IndexSet;
    use std::collections::hash_map::RandomState;

    #[test]
    fn test_with_hasher() {
        let hasher = RandomState::new();
        let set: IndexSet<i32, RandomState> = IndexSet::with_hasher(hasher);
        assert!(set.is_empty());
    }

    #[test]
    fn test_with_hasher_non_empty() {
        let hasher = RandomState::new();
        let mut set: IndexSet<i32, RandomState> = IndexSet::with_hasher(hasher);
        set.insert(1);
        set.insert(2);
        
        assert_eq!(set.len(), 2);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
    }

    #[test]
    fn test_with_hasher_equality() {
        let hasher1 = RandomState::new();
        let set1: IndexSet<i32, RandomState> = IndexSet::with_hasher(hasher1);
        let hasher2 = RandomState::new();
        let set2: IndexSet<i32, RandomState> = IndexSet::with_hasher(hasher2);
        
        assert_eq!(set1, set2);
    }
}

#[cfg(test)]
mod tests_llm_16_676 {
    use crate::IndexSet;

    #[test]
    fn test_index_set_new() {
        let index_set: IndexSet<i32> = IndexSet::new();
        assert!(index_set.is_empty());
        assert_eq!(index_set.len(), 0);
    }
}

#[cfg(test)]
mod tests_llm_16_677 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_with_capacity_zero() {
        let set: IndexSet<u32> = IndexSet::with_capacity(0);
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_with_capacity_non_zero() {
        let set: IndexSet<u32> = IndexSet::with_capacity(5);
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
        assert!(set.capacity() >= 5);
    }

    #[test]
    fn test_with_capacity_non_zero_multiple() {
        let set: IndexSet<u32> = IndexSet::with_capacity(10);
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
        assert!(set.capacity() >= 10);
    }
}
