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
