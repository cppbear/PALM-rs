// Answer 0

#[test]
fn test_entry_insert_and_remove() {
    use hashbrown::hash_table::{Entry, OccupiedEntry, VacantEntry};
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    // Insert an entry
    table.entry(hasher(&1), |val| val.0 == 1, |val| hasher(&val.0)).insert((1, "a"));
    
    // Verify the entry is present
    if let Entry::Occupied(entry) = table.entry(hasher(&1), |val| val.0 == 1, |val| hasher(&val.0)) {
        entry.remove();
    }
    
    // Ensure the entry has been removed
    assert_eq!(table.find(hasher(&1), |val| val.0 == 1), None);
}

#[test]
fn test_entry_vacant_insert() {
    use hashbrown::hash_table::{Entry, VacantEntry};
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Attempt to find an entry that does not exist and insert it
    if let Entry::Vacant(entry) = table.entry(hasher(&2), |val| val.0 == 2, |val| hasher(&val.0)) {
        entry.insert((2, "b"));
    }

    // Verify the new entry is present
    assert_eq!(table.find(hasher(&2), |val| val.0 == 2), Some(&(2, "b")));
}

#[test]
fn test_entry_reinsert_after_removal() {
    use hashbrown::hash_table::{Entry, OccupiedEntry, VacantEntry};
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Insert and remove an entry
    table.entry(hasher(&3), |val| val.0 == 3, |val| hasher(&val.0)).insert((3, "c"));
    if let Entry::Occupied(entry) = table.entry(hasher(&3), |val| val.0 == 3, |val| hasher(&val.0)) {
        entry.remove();
    }

    // Reinsert the same entry
    if let Entry::Vacant(entry) = table.entry(hasher(&3), |val| val.0 == 3, |val| hasher(&val.0)) {
        entry.insert((3, "c"));
    }

    // Verify the entry is still present
    assert_eq!(table.find(hasher(&3), |val| val.0 == 3), Some(&(3, "c")));
}

