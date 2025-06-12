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
    T: Eq + Hash,
    S: BuildHasher,
{
    pub fn is_disjoint<S2>(&self, other: &IndexSet<T, S2>) -> bool
    where
        S2: BuildHasher,
    {}
    pub fn is_subset<S2>(&self, other: &IndexSet<T, S2>) -> bool
    where
        S2: BuildHasher,
    {}
    pub fn is_superset<S2>(&self, other: &IndexSet<T, S2>) -> bool
    where
        S2: BuildHasher,
    {
        other.is_subset(self)
    }
}
