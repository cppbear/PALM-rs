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
#[derive(Copy, Debug)]
struct Bucket<K, V> {
    hash: HashValue,
    key: K,
    value: V,
}
fn reserve_entries<K, V>(
    entries: &mut Entries<K, V>,
    additional: usize,
    try_capacity: usize,
) {
    let try_capacity = try_capacity.min(IndexMapCore::<K, V>::MAX_ENTRIES_CAPACITY);
    let try_add = try_capacity - entries.len();
    if try_add > additional && entries.try_reserve_exact(try_add).is_ok() {
        return;
    }
    entries.reserve_exact(additional);
}
