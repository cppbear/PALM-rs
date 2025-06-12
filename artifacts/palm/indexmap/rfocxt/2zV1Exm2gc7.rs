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
pub struct Drain<'a, K, V> {
    iter: vec::Drain<'a, Bucket<K, V>>,
}
#[derive(Copy, Debug)]
struct Bucket<K, V> {
    hash: HashValue,
    key: K,
    value: V,
}
impl<'a, T> Drain<'a, T> {
    pub(super) fn new(iter: vec::Drain<'a, Bucket<T>>) -> Self {
        Self { iter }
    }
    pub fn as_slice(&self) -> &Slice<T> {}
}
