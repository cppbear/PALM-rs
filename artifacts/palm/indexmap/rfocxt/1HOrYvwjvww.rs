type Bucket<T> = super::Bucket<T, ()>;
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
pub trait MutableValues: private::Sealed {
    type Value;
    fn get_full_mut2<Q>(&mut self, value: &Q) -> Option<(usize, &mut Self::Value)>
    where
        Q: ?Sized + Hash + Equivalent<Self::Value>;
    fn get_index_mut2(&mut self, index: usize) -> Option<&mut Self::Value>;
    fn retain2<F>(&mut self, keep: F)
    where
        F: FnMut(&mut Self::Value) -> bool;
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
pub struct IndexSet<T, S> {
    pub(crate) map: IndexMap<T, (), S>,
}
#[repr(transparent)]
pub struct Slice<T> {
    pub(crate) entries: [Bucket<T>],
}
#[cfg(not(feature = "std"))]
pub struct IndexMap<K, V, S> {
    pub(crate) core: IndexMapCore<K, V>,
    hash_builder: S,
}
impl<T, S> IndexSet<T, S> {
    pub fn pop(&mut self) -> Option<T> {}
    pub fn retain<F>(&mut self, mut keep: F)
    where
        F: FnMut(&T) -> bool,
    {}
    pub fn sort(&mut self)
    where
        T: Ord,
    {}
    pub fn sort_by<F>(&mut self, mut cmp: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {}
    pub fn sorted_by<F>(self, mut cmp: F) -> IntoIter<T>
    where
        F: FnMut(&T, &T) -> Ordering,
    {}
    pub fn sort_unstable(&mut self)
    where
        T: Ord,
    {}
    pub fn sort_unstable_by<F>(&mut self, mut cmp: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {}
    pub fn sorted_unstable_by<F>(self, mut cmp: F) -> IntoIter<T>
    where
        F: FnMut(&T, &T) -> Ordering,
    {}
    pub fn sort_by_cached_key<K, F>(&mut self, mut sort_key: F)
    where
        K: Ord,
        F: FnMut(&T) -> K,
    {}
    pub fn binary_search(&self, x: &T) -> Result<usize, usize>
    where
        T: Ord,
    {}
    #[inline]
    pub fn binary_search_by<'a, F>(&'a self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a T) -> Ordering,
    {}
    #[inline]
    pub fn binary_search_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a T) -> B,
        B: Ord,
    {}
    #[must_use]
    pub fn partition_point<P>(&self, pred: P) -> usize
    where
        P: FnMut(&T) -> bool,
    {
        self.as_slice().partition_point(pred)
    }
    pub fn reverse(&mut self) {}
    pub fn as_slice(&self) -> &Slice<T> {
        Slice::from_slice(self.as_entries())
    }
    pub fn into_boxed_slice(self) -> Box<Slice<T>> {}
    pub fn get_index(&self, index: usize) -> Option<&T> {}
    pub fn get_range<R: RangeBounds<usize>>(&self, range: R) -> Option<&Slice<T>> {}
    pub fn first(&self) -> Option<&T> {}
    pub fn last(&self) -> Option<&T> {}
    pub fn swap_remove_index(&mut self, index: usize) -> Option<T> {}
    pub fn shift_remove_index(&mut self, index: usize) -> Option<T> {}
    #[track_caller]
    pub fn move_index(&mut self, from: usize, to: usize) {}
    #[track_caller]
    pub fn swap_indices(&mut self, a: usize, b: usize) {}
}
