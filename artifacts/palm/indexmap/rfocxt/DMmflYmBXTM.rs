use core::hash::{BuildHasher, Hash};
use super::{
    Bucket, Entries, Entry, Equivalent, IndexMap, IndexedEntry, IterMut2, OccupiedEntry,
    VacantEntry,
};
pub trait MutableKeys: private::Sealed {
    type Key;
    type Value;
    fn get_full_mut2<Q>(
        &mut self,
        key: &Q,
    ) -> Option<(usize, &mut Self::Key, &mut Self::Value)>
    where
        Q: ?Sized + Hash + Equivalent<Self::Key>;
    fn get_index_mut2(
        &mut self,
        index: usize,
    ) -> Option<(&mut Self::Key, &mut Self::Value)>;
    fn iter_mut2(&mut self) -> IterMut2<'_, Self::Key, Self::Value>;
    fn retain2<F>(&mut self, keep: F)
    where
        F: FnMut(&mut Self::Key, &mut Self::Value) -> bool;
}
pub trait RawEntryApiV1<K, V, S>: private::Sealed {
    fn raw_entry_v1(&self) -> RawEntryBuilder<'_, K, V, S>;
    fn raw_entry_mut_v1(&mut self) -> RawEntryBuilderMut<'_, K, V, S>;
}
trait Entries {
    type Entry;
    fn into_entries(self) -> Vec<Self::Entry>;
    fn as_entries(&self) -> &[Self::Entry];
    fn as_entries_mut(&mut self) -> &mut [Self::Entry];
    fn with_entries<F>(&mut self, f: F)
    where
        F: FnOnce(&mut [Self::Entry]);
}
pub trait Sealed {}
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
#[derive(Copy, Debug)]
struct Bucket<K, V> {
    hash: HashValue,
    key: K,
    value: V,
}
impl<K, V, S> MutableKeys for IndexMap<K, V, S>
where
    S: BuildHasher,
{
    type Key = K;
    type Value = V;
    fn get_full_mut2<Q>(&mut self, key: &Q) -> Option<(usize, &mut K, &mut V)>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {}
    fn get_index_mut2(&mut self, index: usize) -> Option<(&mut K, &mut V)> {
        self.as_entries_mut().get_mut(index).map(Bucket::muts)
    }
    fn iter_mut2(&mut self) -> IterMut2<'_, Self::Key, Self::Value> {
        IterMut2::new(self.as_entries_mut())
    }
    fn retain2<F>(&mut self, keep: F)
    where
        F: FnMut(&mut K, &mut V) -> bool,
    {}
}
