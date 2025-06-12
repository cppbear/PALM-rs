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
    pub fn as_mut_slice(&mut self) -> &mut Slice<K, V> {}
}
