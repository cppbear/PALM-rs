type Indices = hash_table::HashTable<usize>;
type Entries<K, V> = Vec<Bucket<K, V>>;
use hashbrown::hash_table;
use crate::vec::{self, Vec};
use crate::TryReserveError;
use core::mem;
use core::ops::RangeBounds;
use crate::util::simplify_range;
use crate::{Bucket, Equivalent, HashValue};
pub use entry::{Entry, IndexedEntry, OccupiedEntry, VacantEntry};
trait Entries {
    type Entry;
    fn into_entries(self) -> Vec<Self::Entry>;
    fn as_entries(&self) -> &[Self::Entry];
    fn as_entries_mut(&mut self) -> &mut [Self::Entry];
    fn with_entries<F>(&mut self, f: F)
    where
        F: FnOnce(&mut [Self::Entry]);
}
struct RefMut<'a, K, V> {
    indices: &'a mut Indices,
    entries: &'a mut Entries<K, V>,
}
#[derive(Clone, Copy, Debug, PartialEq)]
struct HashValue(usize);
#[derive(Copy, Debug)]
struct Bucket<K, V> {
    hash: HashValue,
    key: K,
    value: V,
}
impl<'a, K, V> RefMut<'a, K, V> {
    #[inline]
    fn new(indices: &'a mut Indices, entries: &'a mut Entries<K, V>) -> Self {
        Self { indices, entries }
    }
    #[inline]
    fn reserve_entries(&mut self, additional: usize) {}
    fn insert_unique(
        self,
        hash: HashValue,
        key: K,
        value: V,
    ) -> OccupiedEntry<'a, K, V> {}
    fn shift_insert_unique(&mut self, index: usize, hash: HashValue, key: K, value: V) {}
    fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        match self.entries.get(index) {
            Some(entry) => {
                erase_index(self.indices, entry.hash, index);
                Some(self.shift_remove_finish(index))
            }
            None => None,
        }
    }
    fn shift_remove_finish(&mut self, index: usize) -> (K, V) {
        self.decrement_indices(index + 1, self.entries.len());
        let entry = self.entries.remove(index);
        (entry.key, entry.value)
    }
    fn swap_remove_index(&mut self, index: usize) -> Option<(K, V)> {}
    fn swap_remove_finish(&mut self, index: usize) -> (K, V) {}
    fn decrement_indices(&mut self, start: usize, end: usize) {}
    fn increment_indices(&mut self, start: usize, end: usize) {}
    #[track_caller]
    fn move_index(&mut self, from: usize, to: usize) {}
    #[track_caller]
    fn swap_indices(&mut self, a: usize, b: usize) {}
}
#[inline]
fn erase_index(table: &mut Indices, hash: HashValue, index: usize) {
    if let Ok(entry) = table.find_entry(hash.get(), move |&i| i == index) {
        entry.remove();
    } else if cfg!(debug_assertions) {
        panic!("index not found");
    }
}
