use super::{
    Bucket, Entries, IndexMap, IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys,
    Values, ValuesMut,
};
use crate::util::{slice_eq, try_simplify_range};
use crate::GetDisjointMutError;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::{self, Bound, Index, IndexMut, RangeBounds};
macro_rules! impl_index {
    ($($range:ty),*) => {
        $(impl < K, V, S > Index <$range > for IndexMap < K, V, S > { type Output = Slice
        < K, V >; fn index(& self, range : $range) -> & Self::Output {
        Slice::from_slice(& self.as_entries() [range]) } } impl < K, V, S > IndexMut
        <$range > for IndexMap < K, V, S > { fn index_mut(& mut self, range : $range) ->
        & mut Self::Output { Slice::from_mut_slice(& mut self.as_entries_mut() [range]) }
        } impl < K, V > Index <$range > for Slice < K, V > { type Output = Slice < K, V
        >; fn index(& self, range : $range) -> & Self { Self::from_slice(& self
        .entries[range]) } } impl < K, V > IndexMut <$range > for Slice < K, V > { fn
        index_mut(& mut self, range : $range) -> & mut Self { Self::from_mut_slice(& mut
        self.entries[range]) } })*
    };
}
impl_index!(
    ops::Range < usize >, ops::RangeFrom < usize >, ops::RangeFull, ops::RangeInclusive <
    usize >, ops::RangeTo < usize >, ops::RangeToInclusive < usize >, (Bound < usize >,
    Bound < usize >)
);
#[repr(transparent)]
pub struct Slice<K, V> {
    pub(crate) entries: [Bucket<K, V>],
}
#[derive(Copy, Debug)]
struct Bucket<K, V> {
    hash: HashValue,
    key: K,
    value: V,
}
impl<K, V> Slice<K, V> {
    pub(crate) fn into_entries(self: Box<Self>) -> Vec<Bucket<K, V>> {}
    pub const fn new<'a>() -> &'a Self {
        Self::from_slice(&[])
    }
    pub fn new_mut<'a>() -> &'a mut Self {
        Self::from_mut_slice(&mut [])
    }
    #[inline]
    pub const fn len(&self) -> usize {}
    #[inline]
    pub const fn is_empty(&self) -> bool {}
    pub fn get_index(&self, index: usize) -> Option<(&K, &V)> {}
    pub fn get_index_mut(&mut self, index: usize) -> Option<(&K, &mut V)> {}
    pub fn get_range<R: RangeBounds<usize>>(&self, range: R) -> Option<&Self> {
        let range = try_simplify_range(range, self.entries.len())?;
        self.entries.get(range).map(Slice::from_slice)
    }
    pub fn get_range_mut<R: RangeBounds<usize>>(
        &mut self,
        range: R,
    ) -> Option<&mut Self> {
        let range = try_simplify_range(range, self.entries.len())?;
        self.entries.get_mut(range).map(Slice::from_mut_slice)
    }
    pub fn first(&self) -> Option<(&K, &V)> {}
    pub fn first_mut(&mut self) -> Option<(&K, &mut V)> {
        self.entries.first_mut().map(Bucket::ref_mut)
    }
    pub fn last(&self) -> Option<(&K, &V)> {}
    pub fn last_mut(&mut self) -> Option<(&K, &mut V)> {}
    pub fn split_at(&self, index: usize) -> (&Self, &Self) {
        let (first, second) = self.entries.split_at(index);
        (Self::from_slice(first), Self::from_slice(second))
    }
    pub fn split_at_mut(&mut self, index: usize) -> (&mut Self, &mut Self) {
        let (first, second) = self.entries.split_at_mut(index);
        (Self::from_mut_slice(first), Self::from_mut_slice(second))
    }
    pub fn split_first(&self) -> Option<((&K, &V), &Self)> {
        if let [first, rest @ ..] = &self.entries {
            Some((first.refs(), Self::from_slice(rest)))
        } else {
            None
        }
    }
    pub fn split_first_mut(&mut self) -> Option<((&K, &mut V), &mut Self)> {
        if let [first, rest @ ..] = &mut self.entries {
            Some((first.ref_mut(), Self::from_mut_slice(rest)))
        } else {
            None
        }
    }
    pub fn split_last(&self) -> Option<((&K, &V), &Self)> {
        if let [rest @ .., last] = &self.entries {
            Some((last.refs(), Self::from_slice(rest)))
        } else {
            None
        }
    }
    pub fn split_last_mut(&mut self) -> Option<((&K, &mut V), &mut Self)> {
        if let [rest @ .., last] = &mut self.entries {
            Some((last.ref_mut(), Self::from_mut_slice(rest)))
        } else {
            None
        }
    }
    pub fn iter(&self) -> Iter<'_, K, V> {}
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {}
    pub fn keys(&self) -> Keys<'_, K, V> {}
    pub fn into_keys(self: Box<Self>) -> IntoKeys<K, V> {}
    pub fn values(&self) -> Values<'_, K, V> {}
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {}
    pub fn into_values(self: Box<Self>) -> IntoValues<K, V> {}
    pub fn binary_search_keys(&self, x: &K) -> Result<usize, usize>
    where
        K: Ord,
    {}
    #[inline]
    pub fn binary_search_by<'a, F>(&'a self, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a K, &'a V) -> Ordering,
    {}
    #[inline]
    pub fn binary_search_by_key<'a, B, F>(
        &'a self,
        b: &B,
        mut f: F,
    ) -> Result<usize, usize>
    where
        F: FnMut(&'a K, &'a V) -> B,
        B: Ord,
    {}
    #[must_use]
    pub fn partition_point<P>(&self, mut pred: P) -> usize
    where
        P: FnMut(&K, &V) -> bool,
    {}
    pub fn get_disjoint_mut<const N: usize>(
        &mut self,
        indices: [usize; N],
    ) -> Result<[(&K, &mut V); N], GetDisjointMutError> {}
    #[allow(unsafe_code)]
    pub(crate) fn get_disjoint_opt_mut<const N: usize>(
        &mut self,
        indices: [Option<usize>; N],
    ) -> Result<[Option<(&K, &mut V)>; N], GetDisjointMutError> {}
}
