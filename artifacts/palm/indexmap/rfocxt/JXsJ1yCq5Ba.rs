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
#[derive(Clone, Copy, Debug, PartialEq)]
struct HashValue(usize);
impl HashValue {
    #[inline(always)]
    fn get(self) -> u64 {
        self.0 as u64
    }
}
#[inline]
fn update_index(table: &mut Indices, hash: HashValue, old: usize, new: usize) {
    let index = table.find_mut(hash.get(), move |&i| i == old).expect("index not found");
    *index = new;
}
