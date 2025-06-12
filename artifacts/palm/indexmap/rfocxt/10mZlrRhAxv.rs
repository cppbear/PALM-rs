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
#[derive(Clone)]
pub struct IntoIter<K, V> {
    iter: vec::IntoIter<Bucket<K, V>>,
}
#[derive(Copy, Debug)]
struct Bucket<K, V> {
    hash: HashValue,
    key: K,
    value: V,
}
impl<T> IntoIter<T> {
    pub(super) fn new(entries: Vec<Bucket<T>>) -> Self {
        Self { iter: entries.into_iter() }
    }
    pub fn as_slice(&self) -> &Slice<T> {}
}
