use super::core::IndexMapCore;
use super::{Bucket, Entries, IndexMap, Slice};
use alloc::vec::{self, Vec};
use core::fmt;
use core::hash::{BuildHasher, Hash};
use core::iter::FusedIterator;
use core::ops::{Index, RangeBounds};
use core::slice;
pub struct IterMut<'a, K, V> {
    iter: slice::IterMut<'a, Bucket<K, V>>,
}
#[derive(Copy, Debug)]
struct Bucket<K, V> {
    hash: HashValue,
    key: K,
    value: V,
}
impl<'a, K, V> IterMut<'a, K, V> {
    pub(super) fn new(entries: &'a mut [Bucket<K, V>]) -> Self {
        Self { iter: entries.iter_mut() }
    }
    pub fn as_slice(&self) -> &Slice<K, V> {}
    pub fn into_slice(self) -> &'a mut Slice<K, V> {}
}
