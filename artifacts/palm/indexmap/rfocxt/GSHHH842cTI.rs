use super::{Bucket, Entries, IndexSet, Slice};
use alloc::vec::{self, Vec};
use core::fmt;
use core::hash::{BuildHasher, Hash};
use core::iter::{Chain, FusedIterator};
use core::ops::RangeBounds;
use core::slice::Iter as SliceIter;
pub struct SymmetricDifference<'a, T, S1, S2> {
    iter: Chain<Difference<'a, T, S2>, Difference<'a, T, S1>>,
}
pub struct Difference<'a, T, S> {
    iter: Iter<'a, T>,
    other: &'a IndexSet<T, S>,
}
#[cfg(not(feature = "std"))]
pub struct IndexSet<T, S> {
    pub(crate) map: IndexMap<T, (), S>,
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
        Self { iter: diff1.chain(diff2) }
    }
}
