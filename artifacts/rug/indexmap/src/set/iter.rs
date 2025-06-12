use super::{Bucket, Entries, IndexSet, Slice};

use alloc::vec::{self, Vec};
use core::fmt;
use core::hash::{BuildHasher, Hash};
use core::iter::{Chain, FusedIterator};
use core::ops::RangeBounds;
use core::slice::Iter as SliceIter;

impl<'a, T, S> IntoIterator for &'a IndexSet<T, S> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T, S> IntoIterator for IndexSet<T, S> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.into_entries())
    }
}

/// An iterator over the items of an [`IndexSet`].
///
/// This `struct` is created by the [`IndexSet::iter`] method.
/// See its documentation for more.
pub struct Iter<'a, T> {
    iter: SliceIter<'a, Bucket<T>>,
}

impl<'a, T> Iter<'a, T> {
    pub(super) fn new(entries: &'a [Bucket<T>]) -> Self {
        Self {
            iter: entries.iter(),
        }
    }

    /// Returns a slice of the remaining entries in the iterator.
    pub fn as_slice(&self) -> &'a Slice<T> {
        Slice::from_slice(self.iter.as_slice())
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    iterator_methods!(Bucket::key_ref);
}

impl<T> DoubleEndedIterator for Iter<'_, T> {
    double_ended_iterator_methods!(Bucket::key_ref);
}

impl<T> ExactSizeIterator for Iter<'_, T> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<T> FusedIterator for Iter<'_, T> {}

impl<T> Clone for Iter<'_, T> {
    fn clone(&self) -> Self {
        Iter {
            iter: self.iter.clone(),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for Iter<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<T> Default for Iter<'_, T> {
    fn default() -> Self {
        Self { iter: [].iter() }
    }
}

/// An owning iterator over the items of an [`IndexSet`].
///
/// This `struct` is created by the [`IndexSet::into_iter`] method
/// (provided by the [`IntoIterator`] trait). See its documentation for more.
#[derive(Clone)]
pub struct IntoIter<T> {
    iter: vec::IntoIter<Bucket<T>>,
}

impl<T> IntoIter<T> {
    pub(super) fn new(entries: Vec<Bucket<T>>) -> Self {
        Self {
            iter: entries.into_iter(),
        }
    }

    /// Returns a slice of the remaining entries in the iterator.
    pub fn as_slice(&self) -> &Slice<T> {
        Slice::from_slice(self.iter.as_slice())
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    iterator_methods!(Bucket::key);
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    double_ended_iterator_methods!(Bucket::key);
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<T> FusedIterator for IntoIter<T> {}

impl<T: fmt::Debug> fmt::Debug for IntoIter<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iter = self.iter.as_slice().iter().map(Bucket::key_ref);
        f.debug_list().entries(iter).finish()
    }
}

impl<T> Default for IntoIter<T> {
    fn default() -> Self {
        Self {
            iter: Vec::new().into_iter(),
        }
    }
}

/// A draining iterator over the items of an [`IndexSet`].
///
/// This `struct` is created by the [`IndexSet::drain`] method.
/// See its documentation for more.
pub struct Drain<'a, T> {
    iter: vec::Drain<'a, Bucket<T>>,
}

impl<'a, T> Drain<'a, T> {
    pub(super) fn new(iter: vec::Drain<'a, Bucket<T>>) -> Self {
        Self { iter }
    }

    /// Returns a slice of the remaining entries in the iterator.
    pub fn as_slice(&self) -> &Slice<T> {
        Slice::from_slice(self.iter.as_slice())
    }
}

impl<T> Iterator for Drain<'_, T> {
    type Item = T;

    iterator_methods!(Bucket::key);
}

impl<T> DoubleEndedIterator for Drain<'_, T> {
    double_ended_iterator_methods!(Bucket::key);
}

impl<T> ExactSizeIterator for Drain<'_, T> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<T> FusedIterator for Drain<'_, T> {}

impl<T: fmt::Debug> fmt::Debug for Drain<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iter = self.iter.as_slice().iter().map(Bucket::key_ref);
        f.debug_list().entries(iter).finish()
    }
}

/// A lazy iterator producing elements in the difference of [`IndexSet`]s.
///
/// This `struct` is created by the [`IndexSet::difference`] method.
/// See its documentation for more.
pub struct Difference<'a, T, S> {
    iter: Iter<'a, T>,
    other: &'a IndexSet<T, S>,
}

impl<'a, T, S> Difference<'a, T, S> {
    pub(super) fn new<S1>(set: &'a IndexSet<T, S1>, other: &'a IndexSet<T, S>) -> Self {
        Self {
            iter: set.iter(),
            other,
        }
    }
}

impl<'a, T, S> Iterator for Difference<'a, T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next() {
            if !self.other.contains(item) {
                return Some(item);
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.iter.size_hint().1)
    }
}

impl<T, S> DoubleEndedIterator for Difference<'_, T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next_back() {
            if !self.other.contains(item) {
                return Some(item);
            }
        }
        None
    }
}

impl<T, S> FusedIterator for Difference<'_, T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
}

impl<T, S> Clone for Difference<'_, T, S> {
    fn clone(&self) -> Self {
        Difference {
            iter: self.iter.clone(),
            ..*self
        }
    }
}

impl<T, S> fmt::Debug for Difference<'_, T, S>
where
    T: fmt::Debug + Eq + Hash,
    S: BuildHasher,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

/// A lazy iterator producing elements in the intersection of [`IndexSet`]s.
///
/// This `struct` is created by the [`IndexSet::intersection`] method.
/// See its documentation for more.
pub struct Intersection<'a, T, S> {
    iter: Iter<'a, T>,
    other: &'a IndexSet<T, S>,
}

impl<'a, T, S> Intersection<'a, T, S> {
    pub(super) fn new<S1>(set: &'a IndexSet<T, S1>, other: &'a IndexSet<T, S>) -> Self {
        Self {
            iter: set.iter(),
            other,
        }
    }
}

impl<'a, T, S> Iterator for Intersection<'a, T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next() {
            if self.other.contains(item) {
                return Some(item);
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.iter.size_hint().1)
    }
}

impl<T, S> DoubleEndedIterator for Intersection<'_, T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next_back() {
            if self.other.contains(item) {
                return Some(item);
            }
        }
        None
    }
}

impl<T, S> FusedIterator for Intersection<'_, T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
}

impl<T, S> Clone for Intersection<'_, T, S> {
    fn clone(&self) -> Self {
        Intersection {
            iter: self.iter.clone(),
            ..*self
        }
    }
}

impl<T, S> fmt::Debug for Intersection<'_, T, S>
where
    T: fmt::Debug + Eq + Hash,
    S: BuildHasher,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

/// A lazy iterator producing elements in the symmetric difference of [`IndexSet`]s.
///
/// This `struct` is created by the [`IndexSet::symmetric_difference`] method.
/// See its documentation for more.
pub struct SymmetricDifference<'a, T, S1, S2> {
    iter: Chain<Difference<'a, T, S2>, Difference<'a, T, S1>>,
}

impl<'a, T, S1, S2> SymmetricDifference<'a, T, S1, S2>
where
    T: Eq + Hash,
    S1: BuildHasher,
    S2: BuildHasher,
{
    pub(super) fn new(set1: &'a IndexSet<T, S1>, set2: &'a IndexSet<T, S2>) -> Self {
        let diff1 = set1.difference(set2);
        let diff2 = set2.difference(set1);
        Self {
            iter: diff1.chain(diff2),
        }
    }
}

impl<'a, T, S1, S2> Iterator for SymmetricDifference<'a, T, S1, S2>
where
    T: Eq + Hash,
    S1: BuildHasher,
    S2: BuildHasher,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn fold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        self.iter.fold(init, f)
    }
}

impl<T, S1, S2> DoubleEndedIterator for SymmetricDifference<'_, T, S1, S2>
where
    T: Eq + Hash,
    S1: BuildHasher,
    S2: BuildHasher,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }

    fn rfold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        self.iter.rfold(init, f)
    }
}

impl<T, S1, S2> FusedIterator for SymmetricDifference<'_, T, S1, S2>
where
    T: Eq + Hash,
    S1: BuildHasher,
    S2: BuildHasher,
{
}

impl<T, S1, S2> Clone for SymmetricDifference<'_, T, S1, S2> {
    fn clone(&self) -> Self {
        SymmetricDifference {
            iter: self.iter.clone(),
        }
    }
}

impl<T, S1, S2> fmt::Debug for SymmetricDifference<'_, T, S1, S2>
where
    T: fmt::Debug + Eq + Hash,
    S1: BuildHasher,
    S2: BuildHasher,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

/// A lazy iterator producing elements in the union of [`IndexSet`]s.
///
/// This `struct` is created by the [`IndexSet::union`] method.
/// See its documentation for more.
pub struct Union<'a, T, S> {
    iter: Chain<Iter<'a, T>, Difference<'a, T, S>>,
}

impl<'a, T, S> Union<'a, T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    pub(super) fn new<S2>(set1: &'a IndexSet<T, S>, set2: &'a IndexSet<T, S2>) -> Self
    where
        S2: BuildHasher,
    {
        Self {
            iter: set1.iter().chain(set2.difference(set1)),
        }
    }
}

impl<'a, T, S> Iterator for Union<'a, T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn fold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        self.iter.fold(init, f)
    }
}

impl<T, S> DoubleEndedIterator for Union<'_, T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }

    fn rfold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        self.iter.rfold(init, f)
    }
}

impl<T, S> FusedIterator for Union<'_, T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
}

impl<T, S> Clone for Union<'_, T, S> {
    fn clone(&self) -> Self {
        Union {
            iter: self.iter.clone(),
        }
    }
}

impl<T, S> fmt::Debug for Union<'_, T, S>
where
    T: fmt::Debug + Eq + Hash,
    S: BuildHasher,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

/// A splicing iterator for `IndexSet`.
///
/// This `struct` is created by [`IndexSet::splice()`].
/// See its documentation for more.
pub struct Splice<'a, I, T, S>
where
    I: Iterator<Item = T>,
    T: Hash + Eq,
    S: BuildHasher,
{
    iter: crate::map::Splice<'a, UnitValue<I>, T, (), S>,
}

impl<'a, I, T, S> Splice<'a, I, T, S>
where
    I: Iterator<Item = T>,
    T: Hash + Eq,
    S: BuildHasher,
{
    #[track_caller]
    pub(super) fn new<R>(set: &'a mut IndexSet<T, S>, range: R, replace_with: I) -> Self
    where
        R: RangeBounds<usize>,
    {
        Self {
            iter: set.map.splice(range, UnitValue(replace_with)),
        }
    }
}

impl<I, T, S> Iterator for Splice<'_, I, T, S>
where
    I: Iterator<Item = T>,
    T: Hash + Eq,
    S: BuildHasher,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.iter.next()?.0)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<I, T, S> DoubleEndedIterator for Splice<'_, I, T, S>
where
    I: Iterator<Item = T>,
    T: Hash + Eq,
    S: BuildHasher,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        Some(self.iter.next_back()?.0)
    }
}

impl<I, T, S> ExactSizeIterator for Splice<'_, I, T, S>
where
    I: Iterator<Item = T>,
    T: Hash + Eq,
    S: BuildHasher,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<I, T, S> FusedIterator for Splice<'_, I, T, S>
where
    I: Iterator<Item = T>,
    T: Hash + Eq,
    S: BuildHasher,
{
}

struct UnitValue<I>(I);

impl<I: Iterator> Iterator for UnitValue<I> {
    type Item = (I::Item, ());

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| (x, ()))
    }
}

impl<I, T, S> fmt::Debug for Splice<'_, I, T, S>
where
    I: fmt::Debug + Iterator<Item = T>,
    T: fmt::Debug + Hash + Eq,
    S: BuildHasher,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.iter, f)
    }
}

impl<I: fmt::Debug> fmt::Debug for UnitValue<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

#[cfg(test)]
mod tests_llm_16_218 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_clone() {
        // Create an IndexMap with some data
        let mut original = IndexMap::new();
        original.insert("key1", "value1");
        original.insert("key2", "value2");

        // Clone the original map
        let cloned = original.clone();

        // Assert that the clone contains the same data
        assert_eq!(original.len(), cloned.len());
        assert_eq!(original.get("key1"), cloned.get("key1"));
        assert_eq!(original.get("key2"), cloned.get("key2"));

        // Ensure that the original and clone are different instances
        assert!(!std::ptr::eq(&original, &cloned));
    }
}

#[cfg(test)]
mod tests_llm_16_220 {
    use super::*;

use crate::*;
    use crate::IndexMap;
    use crate::IndexSet;

    #[test]
    fn test_next_back_simple() {
        let mut set = IndexSet::from([1, 2, 3, 4]);
        let other_set = IndexSet::from([2, 4]);
        let mut difference_iter = set.difference(&other_set);

        assert_eq!(difference_iter.next_back(), Some(&1));
        assert_eq!(difference_iter.next_back(), Some(&3));
        assert_eq!(difference_iter.next_back(), None);
    }

    #[test]
    fn test_next_back_empty() {
        let set: IndexSet<i32> = IndexSet::new();
        let other_set: IndexSet<i32> = IndexSet::new();
        let mut difference_iter = set.difference(&other_set);

        assert_eq!(difference_iter.next_back(), None);
    }

    #[test]
    fn test_next_back_no_difference() {
        let set = IndexSet::from([1, 2, 3, 4]);
        let other_set = IndexSet::from([1, 2, 3, 4]);
        let mut difference_iter = set.difference(&other_set);

        assert_eq!(difference_iter.next_back(), None);
    }

    #[test]
    fn test_next_back_with_multiple_elements() {
        let mut set = IndexSet::from([1, 2, 3, 4, 5]);
        let other_set = IndexSet::from([3, 4, 5]);
        let mut difference_iter = set.difference(&other_set);

        assert_eq!(difference_iter.next_back(), Some(&1));
        assert_eq!(difference_iter.next_back(), Some(&2));
        assert_eq!(difference_iter.next_back(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_222 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_size_hint_empty() {
        let map: IndexMap<i32, i32> = IndexMap::new();
        let iter = map.iter();
        let (lower, upper) = iter.size_hint();
        assert_eq!(lower, 0);
        assert_eq!(upper, Some(0));
    }

    #[test]
    fn test_size_hint_non_empty() {
        let mut map = IndexMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        let iter = map.iter();
        let (lower, upper) = iter.size_hint();
        assert_eq!(lower, 2);
        assert_eq!(upper, Some(2));
    }

    #[test]
    fn test_size_hint_with_capacity() {
        let mut map: IndexMap<i32, i32> = IndexMap::with_capacity(10);
        map.insert(1, 10);
        map.insert(2, 20);
        let iter = map.iter();
        let (lower, upper) = iter.size_hint();
        assert_eq!(lower, 2);
        assert_eq!(upper, Some(2));
    }

    #[test]
    fn test_size_hint_after_removal() {
        let mut map = IndexMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        map.remove(&1);
        let iter = map.iter();
        let (lower, upper) = iter.size_hint();
        assert_eq!(lower, 1);
        assert_eq!(upper, Some(1));
    }
}

#[cfg(test)]
mod tests_llm_16_233 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[derive(Clone)]
    struct MyStruct {
        value: i32,
    }

    #[test]
    fn test_intersection_clone() {
        // Creating an example IndexMap
        let mut map: IndexMap<i32, MyStruct> = IndexMap::new();
        map.insert(1, MyStruct { value: 10 });
        map.insert(2, MyStruct { value: 20 });

        // Cloning the map
        let cloned_map = map.clone();

        // Testing the cloned map for integrity
        assert_eq!(map.len(), cloned_map.len());
        assert_eq!(map[&1].value, cloned_map[&1].value);
        assert_eq!(map[&2].value, cloned_map[&2].value);
    }

    #[test]
    fn test_intersection_clone_empty() {
        // Creating an empty IndexMap
        let map: IndexMap<i32, MyStruct> = IndexMap::new();

        // Cloning the empty map
        let cloned_map = map.clone();

        // Testing the cloned empty map
        assert_eq!(map.len(), cloned_map.len());
    }
}

#[cfg(test)]
mod tests_llm_16_238 {
    use super::*;

use crate::*;
    use std::vec;
    use crate::set::iter::IntoIter;

    #[test]
    fn test_into_iter_default() {
        let iter: IntoIter<i32> = IntoIter::default();
        assert_eq!(iter.len(), 0);
    }
}

#[cfg(test)]
mod tests_llm_16_250 {
    use super::*;

use crate::*;
    use crate::set::iter::Iter;

    #[test]
    fn test_iter_default() {
        let iter: Iter<i32> = Iter::default();
        assert_eq!(iter.len(), 0);
    }
}

#[cfg(test)]
mod tests_llm_16_254 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_len() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        
        let iter = set.iter();
        assert_eq!(iter.len(), 3);
        
        set.remove(&2);
        let iter = set.iter();
        assert_eq!(iter.len(), 2);
        
        set.clear();
        let iter = set.iter();
        assert_eq!(iter.len(), 0);
    }
}

#[cfg(test)]
mod tests_llm_16_268 {
    use super::*;

use crate::*;
    use crate::IndexSet;
    use std::collections::hash_map::RandomState;

    #[test]
    fn test_next_back() {
        let set1: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![1, 2, 3]);
        let set2: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![2, 3, 4]);
        
        let symmetric_diff = SymmetricDifference::new(&set1, &set2);
        let mut iter = symmetric_diff;

        assert_eq!(iter.next_back(), Some(&4)); // Expect 4 from set2
        assert_eq!(iter.next_back(), Some(&1)); // Expect 1 from set1
        assert_eq!(iter.next_back(), None);      // No more elements
    }
}

#[cfg(test)]
mod tests_llm_16_269 {
    use super::*;

use crate::*;
    use crate::IndexSet;
    use std::collections::hash_map::RandomState;

    #[test]
    fn test_rfold() {
        let set1: IndexSet<i32, RandomState> = [1, 2, 3].iter().cloned().collect();
        let set2: IndexSet<i32, RandomState> = [2, 3, 4].iter().cloned().collect();
        let symmetric_diff = SymmetricDifference::new(&set1, &set2);
        
        let result = symmetric_diff.rfold(0, |acc, &x| acc + x);
        
        assert_eq!(result, 5); // 1 + 4 = 5
    }
}

#[cfg(test)]
mod tests_llm_16_272 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_size_hint() {
        let set1: IndexSet<i32> = [1, 2, 3].iter().cloned().collect();
        let set2: IndexSet<i32> = [2, 3, 4].iter().cloned().collect();

        let symmetric_diff = SymmetricDifference::new(&set1, &set2);
        let (lower, upper) = symmetric_diff.size_hint();

        assert_eq!(lower, 2);
        assert_eq!(upper, Some(2)); // Since 1 and 4 are the symmetric difference
    }

    #[test]
    fn test_size_hint_empty() {
        let set1: IndexSet<i32> = [].iter().cloned().collect();
        let set2: IndexSet<i32> = [].iter().cloned().collect();

        let symmetric_diff = SymmetricDifference::new(&set1, &set2);
        let (lower, upper) = symmetric_diff.size_hint();

        assert_eq!(lower, 0);
        assert_eq!(upper, Some(0)); // No elements in symmetric difference
    }

    #[test]
    fn test_size_hint_one_element_different() {
        let set1: IndexSet<i32> = [1].iter().cloned().collect();
        let set2: IndexSet<i32> = [2].iter().cloned().collect();

        let symmetric_diff = SymmetricDifference::new(&set1, &set2);
        let (lower, upper) = symmetric_diff.size_hint();

        assert_eq!(lower, 2);
        assert_eq!(upper, Some(2)); // 1 and 2 are both in symmetric difference
    }

    #[test]
    fn test_size_hint_multiple_elements() {
        let set1: IndexSet<i32> = [1, 2, 3].iter().cloned().collect();
        let set2: IndexSet<i32> = [3, 4, 5].iter().cloned().collect();

        let symmetric_diff = SymmetricDifference::new(&set1, &set2);
        let (lower, upper) = symmetric_diff.size_hint();

        assert_eq!(lower, 4);
        assert_eq!(upper, Some(4)); // 1, 2, 4, and 5 are in symmetric difference
    }
}

#[cfg(test)]
mod tests_llm_16_276 {
    use super::*;

use crate::*;
    use crate::IndexSet;
    use std::collections::hash_map::RandomState;

    #[test]
    fn test_rfold() {
        let set1: IndexSet<i32, RandomState> = [1, 2, 3].iter().cloned().collect();
        let set2: IndexSet<i32, RandomState> = [3, 4, 5].iter().cloned().collect();
        let union = Union::new(&set1, &set2);

        let result = union.rfold(0, |acc, &x| acc + x);

        assert_eq!(result, 15); // 1 + 2 + 3 + 4 + 5 = 15
    }
}

#[cfg(test)]
mod tests_llm_16_281 {
    use super::*;

use crate::*;
    use crate::set::iter::UnitValue;

    #[test]
    fn test_next_some() {
        let vec = vec![1, 2, 3];
        let iter = UnitValue(vec.iter());
        let mut unit_value_iter = iter;

        assert_eq!(unit_value_iter.next(), Some((&1, ())));
        assert_eq!(unit_value_iter.next(), Some((&2, ())));
        assert_eq!(unit_value_iter.next(), Some((&3, ())));
    }

    #[test]
    fn test_next_none() {
        let vec: Vec<i32> = vec![];
        let iter = UnitValue(vec.iter());
        let mut unit_value_iter = iter;

        assert_eq!(unit_value_iter.next(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_678 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_into_iter() {
        let mut set = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        let iter = set.into_iter();
        let collected: Vec<_> = iter.collect();

        assert_eq!(collected, vec![1, 2, 3]);
    }

    #[test]
    fn test_into_iter_empty() {
        let set: IndexSet<i32> = IndexSet::new();
        let iter = set.into_iter();
        
        let collected: Vec<_> = iter.collect();
        
        assert!(collected.is_empty());
    }

    #[test]
    fn test_into_iter_order() {
        let mut set = IndexSet::new();
        set.insert(3);
        set.insert(1);
        set.insert(2);

        let iter = set.into_iter();
        let collected: Vec<_> = iter.collect();

        assert_eq!(collected, vec![3, 1, 2]); // Should match insertion order
    }

    #[test]
    fn test_into_iter_duplicates() {
        let mut set = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(1); // Duplicate, should be ignored

        let iter = set.into_iter();
        let collected: Vec<_> = iter.collect();

        assert_eq!(collected, vec![1, 2]); // Should still be unique
    }
}

#[cfg(test)]
mod tests_llm_16_679 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_into_iter() {
        let mut index_map = IndexMap::new();
        index_map.insert(1, "a");
        index_map.insert(2, "b");
        index_map.insert(3, "c");

        let mut iter = index_map.into_iter();
        let mut entries: Vec<_> = iter.collect();

        assert_eq!(entries.len(), 3);
        assert!(entries.contains(&(1, "a")));
        assert!(entries.contains(&(2, "b")));
        assert!(entries.contains(&(3, "c")));
    }

    #[test]
    fn test_into_iter_empty() {
        let index_map: IndexMap<i32, &str> = IndexMap::new();
        let iter: Vec<_> = index_map.into_iter().collect();

        assert!(iter.is_empty());
    }

    #[test]
    fn test_into_iter_order() {
        let mut index_map = IndexMap::new();
        index_map.insert(1, "one");
        index_map.insert(2, "two");
        index_map.insert(3, "three");

        let mut iter = index_map.into_iter();
        let first = iter.next().unwrap();
        let second = iter.next().unwrap();
        let third = iter.next().unwrap();

        assert_eq!(first, (1, "one"));
        assert_eq!(second, (2, "two"));
        assert_eq!(third, (3, "three"));
    }

    #[test]
    fn test_into_iter_mut() {
        let mut index_map = IndexMap::new();
        index_map.insert(1, 10);
        index_map.insert(2, 20);
        index_map.insert(3, 30);

        let mut iter = index_map.into_iter();
        let first = iter.next().unwrap();
        let second = iter.next().unwrap();
        let third = iter.next().unwrap();

        assert_eq!(first, (1, 10));
        assert_eq!(second, (2, 20));
        assert_eq!(third, (3, 30));
    }
}

#[cfg(test)]
mod tests_llm_16_683 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_intersection_new() {
        let set1: IndexSet<u32> = IndexSet::from_iter(vec![1, 2, 3]);
        let set2: IndexSet<u32> = IndexSet::from_iter(vec![2, 3, 4]);
        let intersection = set::iter::Intersection::new(&set1, &set2);

        let intersection_elements: Vec<_> = intersection.iter.collect();
        assert_eq!(intersection_elements, vec![&2, &3]);
    }

    #[test]
    fn test_intersection_new_empty() {
        let set1: IndexSet<u32> = IndexSet::new();
        let set2: IndexSet<u32> = IndexSet::new();
        let intersection = set::iter::Intersection::new(&set1, &set2);

        let intersection_elements: Vec<_> = intersection.iter.collect();
        assert!(intersection_elements.is_empty());
    }

    #[test]
    fn test_intersection_one_empty() {
        let set1: IndexSet<u32> = IndexSet::from_iter(vec![1, 2, 3]);
        let set2: IndexSet<u32> = IndexSet::new();
        let intersection = set::iter::Intersection::new(&set1, &set2);

        let intersection_elements: Vec<_> = intersection.iter.collect();
        assert!(intersection_elements.is_empty());
    }
}
