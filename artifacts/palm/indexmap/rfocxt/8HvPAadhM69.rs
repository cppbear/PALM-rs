use super::{Bucket, Entries, IndexSet, Slice};
use alloc::vec::{self, Vec};
use core::fmt;
use core::hash::{BuildHasher, Hash};
use core::iter::{Chain, FusedIterator};
use core::ops::RangeBounds;
use core::slice::Iter as SliceIter;
pub struct Drain<'a, T> {
    iter: vec::Drain<'a, Bucket<T>>,
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
pub struct Drain<'a, K, V> {
    iter: vec::Drain<'a, Bucket<K, V>>,
}
impl<'a, T> Drain<'a, T> {
    pub(super) fn new(iter: vec::Drain<'a, Bucket<T>>) -> Self {
        Self { iter }
    }
    pub fn as_slice(&self) -> &Slice<T> {
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
