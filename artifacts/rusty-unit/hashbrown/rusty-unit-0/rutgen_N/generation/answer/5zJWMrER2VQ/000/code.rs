// Answer 0

#[test]
fn test_drain_empty_table() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    let drained_elements: Vec<_> = table.drain().collect();
    assert!(drained_elements.is_empty());
    assert!(table.is_empty());
}

#[test]
fn test_drain_non_empty_table() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    for x in 1..=3 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    
    let drained_elements: Vec<_> = table.drain().collect();
    assert_eq!(drained_elements.len(), 3);
    assert!(table.is_empty());
}

#[test]
fn test_drain_table_with_duplicates() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Assuming insert_unique correctly handles duplicates.
    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&2), 2, hasher);
    table.insert_unique(hasher(&2), 2, hasher); // Duplicate insert

    let drained_elements: Vec<_> = table.drain().collect();
    assert_eq!(drained_elements.len(), 2); // Only unique values should be counted
    assert!(table.is_empty());
}

