// Answer 0

#[test]
fn test_clear_table() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hash_fn = |val: &_| hasher.hash_one(val);

    table.insert_unique(hash_fn(&1), 1, hash_fn);
    assert!(!table.is_empty()); // Ensure it is not empty before clear

    table.clear();
    assert!(table.is_empty()); // Should be empty after clear
}

#[test]
fn test_clear_empty_table() {
    use hashbrown::HashTable;

    let mut table: HashTable<i32, i32> = HashTable::new();
    assert!(table.is_empty()); // Ensure it is empty initially

    table.clear(); // Calling clear on an empty table
    assert!(table.is_empty()); // Still should be empty
}

