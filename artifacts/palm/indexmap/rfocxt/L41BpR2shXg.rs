use super::core::IndexMapCore;
use super::{Bucket, Entries, IndexMap, Slice};
use alloc::vec::{self, Vec};
use core::fmt;
use core::hash::{BuildHasher, Hash};
use core::iter::FusedIterator;
use core::ops::{Index, RangeBounds};
use core::slice;
pub struct Values<'a, K, V> {
    iter: slice::Iter<'a, Bucket<K, V>>,
}
pub struct Iter<'a, T> {
    iter: SliceIter<'a, Bucket<T>>,
}
#[derive(Copy, Debug)]
struct Bucket<K, V> {
    hash: HashValue,
    key: K,
    value: V,
}
pub struct Iter<'a, K, V> {
    iter: slice::Iter<'a, Bucket<K, V>>,
}
impl<'a, K, V> Values<'a, K, V> {
    pub(super) fn new(entries: &'a [Bucket<K, V>]) -> Self {
        Self { iter: entries.iter() }
    }
}
