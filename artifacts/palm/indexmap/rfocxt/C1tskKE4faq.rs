use super::{Bucket, Entries, IndexSet, Slice};
use alloc::vec::{self, Vec};
use core::fmt;
use core::hash::{BuildHasher, Hash};
use core::iter::{Chain, FusedIterator};
use core::ops::RangeBounds;
use core::slice::Iter as SliceIter;
struct UnitValue<I>(I);
impl<I: fmt::Debug> fmt::Debug for UnitValue<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}
