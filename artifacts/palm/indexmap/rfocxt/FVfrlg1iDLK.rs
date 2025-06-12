use super::{Bucket, Entries, IndexSet, IntoIter, Iter};
use crate::util::{slice_eq, try_simplify_range};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::{self, Bound, Index, RangeBounds};
macro_rules! impl_index {
    ($($range:ty),*) => {
        $(impl < T, S > Index <$range > for IndexSet < T, S > { type Output = Slice < T
        >; fn index(& self, range : $range) -> & Self::Output { Slice::from_slice(& self
        .as_entries() [range]) } } impl < T > Index <$range > for Slice < T > { type
        Output = Self; fn index(& self, range : $range) -> & Self::Output {
        Slice::from_slice(& self.entries[range]) } })*
    };
}
impl_index!(
    ops::Range < usize >, ops::RangeFrom < usize >, ops::RangeFull, ops::RangeInclusive <
    usize >, ops::RangeTo < usize >, ops::RangeToInclusive < usize >, (Bound < usize >,
    Bound < usize >)
);
#[repr(transparent)]
pub struct Slice<T> {
    pub(crate) entries: [Bucket<T>],
}
#[derive(Copy, Debug)]
struct Bucket<K, V> {
    hash: HashValue,
    key: K,
    value: V,
}
impl<T> Index<usize> for Slice<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index].key
    }
}
