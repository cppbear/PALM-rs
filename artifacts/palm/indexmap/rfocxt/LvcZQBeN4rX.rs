use super::{Bucket, Entries, IndexSet, Slice};
use alloc::vec::{self, Vec};
use core::fmt;
use core::hash::{BuildHasher, Hash};
use core::iter::{Chain, FusedIterator};
use core::ops::RangeBounds;
use core::slice::Iter as SliceIter;
#[derive(Clone)]
pub struct IntoIter<T> {
    iter: vec::IntoIter<Bucket<T>>,
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
#[derive(Clone)]
pub struct IntoIter<K, V> {
    iter: vec::IntoIter<Bucket<K, V>>,
}
impl<T> IntoIter<T> {
    pub(super) fn new(entries: Vec<Bucket<T>>) -> Self {
        Self { iter: entries.into_iter() }
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
