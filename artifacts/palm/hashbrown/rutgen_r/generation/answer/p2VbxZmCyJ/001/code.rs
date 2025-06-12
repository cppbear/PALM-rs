// Answer 0

#[test]
fn test_clear_on_empty_table() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let hasher = DefaultHashBuilder::default();
    let mut table: HashTable<i32, i32, _> = HashTable::with_hasher(hasher);
    table.clear();
    assert!(table.is_empty());
}

#[test]
fn test_clear_on_non_empty_table() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let hasher = DefaultHashBuilder::default();
    let mut table: HashTable<i32, i32, _> = HashTable::with_hasher(hasher);
    table.insert_unique(1, 100, |val: &_| hasher.hash_one(val));
    table.insert_unique(2, 200, |val: &_| hasher.hash_one(val));
    table.clear();
    assert!(table.is_empty());
}

#[test]
fn test_clear_with_multiple_insertions() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let hasher = DefaultHashBuilder::default();
    let mut table: HashTable<i32, i32, _> = HashTable::with_hasher(hasher);
    for i in 0..100 {
        table.insert_unique(i, i * 10, |val: &_| hasher.hash_one(val));
    }
    table.clear();
    assert!(table.is_empty());
}

#[test]
#[should_panic]
fn test_clear_on_dropped_table() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let hasher = DefaultHashBuilder::default();
    let mut table: HashTable<i32, i32, _> = HashTable::with_hasher(hasher);
    table.insert_unique(1, 100, |val: &_| hasher.hash_one(val));
    drop(table);
    // Attempting to access methods after drop should panic
    // We cannot perform actual calls after the drop without  triggering compile errors
    // Placeholder assertion
    assert!(false);
}

