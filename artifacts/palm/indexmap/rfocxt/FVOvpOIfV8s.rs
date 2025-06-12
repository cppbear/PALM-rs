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
impl<T> Slice<T> {
    pub(crate) fn into_entries(self: Box<Self>) -> Vec<Bucket<T>> {}
    pub const fn new<'a>() -> &'a Self {
        Self::from_slice(&[])
    }
    pub const fn len(&self) -> usize {}
    pub const fn is_empty(&self) -> bool {}
    pub fn get_index(&self, index: usize) -> Option<&T> {}
    pub fn get_range<R: RangeBounds<usize>>(&self, range: R) -> Option<&Self> {
        let range = try_simplify_range(range, self.entries.len())?;
        self.entries.get(range).map(Self::from_slice)
    }
    pub fn first(&self) -> Option<&T> {}
    pub fn last(&self) -> Option<&T> {}
    pub fn split_at(&self, index: usize) -> (&Self, &Self) {
        let (first, second) = self.entries.split_at(index);
        (Self::from_slice(first), Self::from_slice(second))
    }
    pub fn split_first(&self) -> Option<(&T, &Self)> {
        if let [first, rest @ ..] = &self.entries {
            Some((&first.key, Self::from_slice(rest)))
        } else {
            None
        }
    }
    pub fn split_last(&self) -> Option<(&T, &Self)> {
        if let [rest @ .., last] = &self.entries {
            Some((&last.key, Self::from_slice(rest)))
        } else {
            None
        }
    }
    pub fn iter(&self) -> Iter<'_, T> {}
    pub fn binary_search(&self, x: &T) -> Result<usize, usize>
    where
        T: Ord,
    {
        self.binary_search_by(|p| p.cmp(x))
    }
    #[inline]
    pub fn binary_search_by<'a, F>(&'a self, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a T) -> Ordering,
    {}
    #[inline]
    pub fn binary_search_by_key<'a, B, F>(
        &'a self,
        b: &B,
        mut f: F,
    ) -> Result<usize, usize>
    where
        F: FnMut(&'a T) -> B,
        B: Ord,
    {}
    #[must_use]
    pub fn partition_point<P>(&self, mut pred: P) -> usize
    where
        P: FnMut(&T) -> bool,
    {}
}
