use super::{Bucket, Entries, IndexSet, Slice};
use alloc::vec::{self, Vec};
use core::fmt;
use core::hash::{BuildHasher, Hash};
use core::iter::{Chain, FusedIterator};
use core::ops::RangeBounds;
use core::slice::Iter as SliceIter;
pub struct Iter<'a, T> {
    iter: SliceIter<'a, Bucket<T>>,
}
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
impl<'a, T> Iter<'a, T> {
    pub(super) fn new(entries: &'a [Bucket<T>]) -> Self {
        Self { iter: entries.iter() }
    }
    pub fn as_slice(&self) -> &'a Slice<T> {
        Slice::from_slice(self.iter.as_slice())
    }
}
#[allow(unsafe_code)]
impl<T> Slice<T> {
    pub(super) const fn from_slice(entries: &[Bucket<T>]) -> &Self {
        unsafe { &*(entries as *const [Bucket<T>] as *const Self) }
    }
    pub(super) fn from_boxed(entries: Box<[Bucket<T>]>) -> Box<Self> {
        unsafe { Box::from_raw(Box::into_raw(entries) as *mut Self) }
    }
    fn into_boxed(self: Box<Self>) -> Box<[Bucket<T>]> {}
}
