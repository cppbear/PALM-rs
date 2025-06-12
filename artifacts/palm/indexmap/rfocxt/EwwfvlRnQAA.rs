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
#[cfg(not(feature = "std"))]
pub struct IndexMap<K, V, S> {
    pub(crate) core: IndexMapCore<K, V>,
    hash_builder: S,
}
#[derive(Debug)]
pub(crate) struct IndexMapCore<K, V> {
    /// indices mapping from the entry hash to its index.
    indices: Indices,
    /// entries is a dense vec maintaining entry order.
    entries: Entries<K, V>,
}
impl<I, K, V, S> fmt::Debug for Splice<'_, I, K, V, S>
where
    I: fmt::Debug + Iterator<Item = (K, V)>,
    K: fmt::Debug + Hash + Eq,
    V: fmt::Debug,
    S: BuildHasher,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Splice")
            .field("drain", &self.drain)
            .field("replace_with", &self.replace_with)
            .finish()
    }
}
