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
    pub fn as_slice(&self) -> &'a Slice<T> {}
}
