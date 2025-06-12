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
    S: BuildHasher,
{
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {}
    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {}
    pub fn get_full<Q>(&self, value: &Q) -> Option<(usize, &T)>
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {}
    pub fn get_index_of<Q>(&self, value: &Q) -> Option<usize>
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {}
    #[deprecated(
        note = "`remove` disrupts the set order -- \
        use `swap_remove` or `shift_remove` for explicit behavior."
    )]
    pub fn remove<Q>(&mut self, value: &Q) -> bool
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {}
    pub fn swap_remove<Q>(&mut self, value: &Q) -> bool
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {}
    pub fn shift_remove<Q>(&mut self, value: &Q) -> bool
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {}
    #[deprecated(
        note = "`take` disrupts the set order -- \
        use `swap_take` or `shift_take` for explicit behavior."
    )]
    pub fn take<Q>(&mut self, value: &Q) -> Option<T>
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {}
    pub fn swap_take<Q>(&mut self, value: &Q) -> Option<T>
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {
        self.map.swap_remove_entry(value).map(|(x, ())| x)
    }
    pub fn shift_take<Q>(&mut self, value: &Q) -> Option<T>
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {}
    pub fn swap_remove_full<Q>(&mut self, value: &Q) -> Option<(usize, T)>
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {}
    pub fn shift_remove_full<Q>(&mut self, value: &Q) -> Option<(usize, T)>
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {}
}
