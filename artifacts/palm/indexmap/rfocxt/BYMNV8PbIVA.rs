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
#[cfg(not(feature = "std"))]
pub struct IndexMap<K, V, S> {
    pub(crate) core: IndexMapCore<K, V>,
    hash_builder: S,
}
impl<T, S> IndexSet<T, S>
where
    T: Hash + Eq,
    S: BuildHasher,
{
    pub fn insert(&mut self, value: T) -> bool {}
    pub fn insert_full(&mut self, value: T) -> (usize, bool) {}
    pub fn insert_sorted(&mut self, value: T) -> (usize, bool)
    where
        T: Ord,
    {}
    #[track_caller]
    pub fn insert_before(&mut self, index: usize, value: T) -> (usize, bool) {}
    #[track_caller]
    pub fn shift_insert(&mut self, index: usize, value: T) -> bool {}
    pub fn replace(&mut self, value: T) -> Option<T> {
        self.replace_full(value).1
    }
    pub fn replace_full(&mut self, value: T) -> (usize, Option<T>) {
        let hash = self.map.hash(&value);
        match self.map.core.replace_full(hash, value, ()) {
            (i, Some((replaced, ()))) => (i, Some(replaced)),
            (i, None) => (i, None),
        }
    }
    pub fn difference<'a, S2>(
        &'a self,
        other: &'a IndexSet<T, S2>,
    ) -> Difference<'a, T, S2>
    where
        S2: BuildHasher,
    {}
    pub fn symmetric_difference<'a, S2>(
        &'a self,
        other: &'a IndexSet<T, S2>,
    ) -> SymmetricDifference<'a, T, S, S2>
    where
        S2: BuildHasher,
    {}
    pub fn intersection<'a, S2>(
        &'a self,
        other: &'a IndexSet<T, S2>,
    ) -> Intersection<'a, T, S2>
    where
        S2: BuildHasher,
    {}
    pub fn union<'a, S2>(&'a self, other: &'a IndexSet<T, S2>) -> Union<'a, T, S>
    where
        S2: BuildHasher,
    {}
    #[track_caller]
    pub fn splice<R, I>(
        &mut self,
        range: R,
        replace_with: I,
    ) -> Splice<'_, I::IntoIter, T, S>
    where
        R: RangeBounds<usize>,
        I: IntoIterator<Item = T>,
    {}
    pub fn append<S2>(&mut self, other: &mut IndexSet<T, S2>) {}
}
