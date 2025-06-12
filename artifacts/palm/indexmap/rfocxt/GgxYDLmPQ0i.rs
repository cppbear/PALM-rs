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
#[derive(Copy, Debug)]
struct Bucket<K, V> {
    hash: HashValue,
    key: K,
    value: V,
}
#[derive(Clone, Copy, Debug, PartialEq)]
struct HashValue(usize);
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
    fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {}
    fn shift_remove_finish(&mut self, index: usize) -> (K, V) {}
    fn swap_remove_index(&mut self, index: usize) -> Option<(K, V)> {}
    fn swap_remove_finish(&mut self, index: usize) -> (K, V) {}
    fn decrement_indices(&mut self, start: usize, end: usize) {
        let shifted_entries = &self.entries[start..end];
        if shifted_entries.len() > self.indices.capacity() / 2 {
            for i in &mut *self.indices {
                if start <= *i && *i < end {
                    *i -= 1;
                }
            }
        } else {
            for (i, entry) in (start..end).zip(shifted_entries) {
                update_index(self.indices, entry.hash, i, i - 1);
            }
        }
    }
    fn increment_indices(&mut self, start: usize, end: usize) {
        let shifted_entries = &self.entries[start..end];
        if shifted_entries.len() > self.indices.capacity() / 2 {
            for i in &mut *self.indices {
                if start <= *i && *i < end {
                    *i += 1;
                }
            }
        } else {
            for (i, entry) in (start..end).zip(shifted_entries).rev() {
                update_index(self.indices, entry.hash, i, i + 1);
            }
        }
    }
    #[track_caller]
    fn move_index(&mut self, from: usize, to: usize) {
        let from_hash = self.entries[from].hash;
        let _ = self.entries[to];
        if from != to {
            update_index(self.indices, from_hash, from, usize::MAX);
            if from < to {
                self.decrement_indices(from + 1, to + 1);
                self.entries[from..=to].rotate_left(1);
            } else if to < from {
                self.increment_indices(to, from);
                self.entries[to..=from].rotate_right(1);
            }
            update_index(self.indices, from_hash, usize::MAX, to);
        }
    }
    #[track_caller]
    fn swap_indices(&mut self, a: usize, b: usize) {}
}
#[inline]
fn update_index(table: &mut Indices, hash: HashValue, old: usize, new: usize) {
    let index = table.find_mut(hash.get(), move |&i| i == old).expect("index not found");
    *index = new;
}
