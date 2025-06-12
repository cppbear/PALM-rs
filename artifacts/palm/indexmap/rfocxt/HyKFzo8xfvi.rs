use super::core::IndexMapCore;
use super::{Bucket, Entries, IndexMap, Slice};
use alloc::vec::{self, Vec};
use core::fmt;
use core::hash::{BuildHasher, Hash};
use core::iter::FusedIterator;
use core::ops::{Index, RangeBounds};
use core::slice;
#[derive(Clone)]
pub struct IntoIter<K, V> {
    iter: vec::IntoIter<Bucket<K, V>>,
}
#[repr(transparent)]
pub struct Slice<K, V> {
    pub(crate) entries: [Bucket<K, V>],
}
#[derive(Clone)]
pub struct IntoIter<T> {
    iter: vec::IntoIter<Bucket<T>>,
}
#[derive(Copy, Debug)]
struct Bucket<K, V> {
    hash: HashValue,
    key: K,
    value: V,
}
impl<K, V> IntoIter<K, V> {
    pub(super) fn new(entries: Vec<Bucket<K, V>>) -> Self {
        Self { iter: entries.into_iter() }
    }
    pub fn as_slice(&self) -> &Slice<K, V> {}
    pub fn as_mut_slice(&mut self) -> &mut Slice<K, V> {
        Slice::from_mut_slice(self.iter.as_mut_slice())
    }
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
    fn into_boxed(self: Box<Self>) -> Box<[Bucket<K, V>]> {}
}
