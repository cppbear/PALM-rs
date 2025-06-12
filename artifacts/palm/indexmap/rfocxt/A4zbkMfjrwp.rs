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
#[allow(unsafe_code)]
impl<K, V> Slice<K, V> {
    pub(super) const fn from_slice(entries: &[Bucket<K, V>]) -> &Self {
        unsafe { &*(entries as *const [Bucket<K, V>] as *const Self) }
    }
    pub(super) fn from_mut_slice(entries: &mut [Bucket<K, V>]) -> &mut Self {
        unsafe { &mut *(entries as *mut [Bucket<K, V>] as *mut Self) }
    }
    pub(super) fn from_boxed(entries: Box<[Bucket<K, V>]>) -> Box<Self> {
        unsafe { Box::from_raw(Box::into_raw(entries) as *mut Self) }
    }
    fn into_boxed(self: Box<Self>) -> Box<[Bucket<K, V>]> {
        unsafe { Box::from_raw(Box::into_raw(self) as *mut [Bucket<K, V>]) }
    }
}
