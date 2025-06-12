use super::{Bucket, Entries, IndexSet, Slice};
use alloc::vec::{self, Vec};
use core::fmt;
use core::hash::{BuildHasher, Hash};
use core::iter::{Chain, FusedIterator};
use core::ops::RangeBounds;
use core::slice::Iter as SliceIter;
pub struct Splice<'a, I, T, S>
where
    I: Iterator<Item = T>,
    T: Hash + Eq,
    S: BuildHasher,
{
    iter: crate::map::Splice<'a, UnitValue<I>, T, (), S>,
}
#[cfg(not(feature = "std"))]
pub struct IndexMap<K, V, S> {
    pub(crate) core: IndexMapCore<K, V>,
    hash_builder: S,
}
#[cfg(not(feature = "std"))]
pub struct IndexSet<T, S> {
    pub(crate) map: IndexMap<T, (), S>,
}
pub struct Splice<'a, I, K, V, S>
where
    I: Iterator<Item = (K, V)>,
    K: Hash + Eq,
    S: BuildHasher,
{
    map: &'a mut IndexMap<K, V, S>,
    tail: IndexMapCore<K, V>,
    drain: vec::IntoIter<Bucket<K, V>>,
    replace_with: I,
}
struct UnitValue<I>(I);
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
