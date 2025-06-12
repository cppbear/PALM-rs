use super::core::IndexMapCore;
use super::{Bucket, Entries, IndexMap, Slice};
use alloc::vec::{self, Vec};
use core::fmt;
use core::hash::{BuildHasher, Hash};
use core::iter::FusedIterator;
use core::ops::{Index, RangeBounds};
use core::slice;
pub struct Splice<'a, I, K, V, S>
where
    I: Iterator<Item = (K, V)>,
    K: Hash + Eq,
    S: BuildHasher,
{
    map: &'a mut IndexMap<K, V, S>,
    tail: IndexMapCore<K, V>,
    drain: vec::IntoIter<Bucket<K, V>>,
    replace_with: I,
}
#[derive(Copy, Debug)]
struct Bucket<K, V> {
    hash: HashValue,
    key: K,
    value: V,
}
#[derive(Debug)]
pub(crate) struct IndexMapCore<K, V> {
    /// indices mapping from the entry hash to its index.
    indices: Indices,
    /// entries is a dense vec maintaining entry order.
    entries: Entries<K, V>,
}
#[cfg(not(feature = "std"))]
pub struct IndexMap<K, V, S> {
    pub(crate) core: IndexMapCore<K, V>,
    hash_builder: S,
}
#[derive(Clone)]
pub struct IntoIter<K, V> {
    iter: vec::IntoIter<Bucket<K, V>>,
}
#[derive(Clone)]
pub struct IntoIter<T> {
    iter: vec::IntoIter<Bucket<T>>,
}
impl<'a, I, K, V, S> Splice<'a, I, K, V, S>
where
    I: Iterator<Item = (K, V)>,
    K: Hash + Eq,
    S: BuildHasher,
{
    #[track_caller]
    pub(super) fn new<R>(
        map: &'a mut IndexMap<K, V, S>,
        range: R,
        replace_with: I,
    ) -> Self
    where
        R: RangeBounds<usize>,
    {
        let (tail, drain) = map.core.split_splice(range);
        Self {
            map,
            tail,
            drain,
            replace_with,
        }
    }
}
