use super::core::IndexMapCore;
use super::{Bucket, Entries, IndexMap, Slice};
use alloc::vec::{self, Vec};
use core::fmt;
use core::hash::{BuildHasher, Hash};
use core::iter::FusedIterator;
use core::ops::{Index, RangeBounds};
use core::slice;
pub struct Drain<'a, K, V> {
    iter: vec::Drain<'a, Bucket<K, V>>,
}
pub struct Drain<'a, T> {
    iter: vec::Drain<'a, Bucket<T>>,
}
#[derive(Copy, Debug)]
struct Bucket<K, V> {
    hash: HashValue,
    key: K,
    value: V,
}
impl<'a, K, V> Drain<'a, K, V> {
    pub(super) fn new(iter: vec::Drain<'a, Bucket<K, V>>) -> Self {
        Self { iter }
    }
    pub fn as_slice(&self) -> &Slice<K, V> {}
}
