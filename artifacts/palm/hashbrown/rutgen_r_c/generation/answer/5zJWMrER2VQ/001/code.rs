// Answer 0

#[test]
fn test_drain_non_empty_table() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new_in(DefaultHashBuilder::default());
    let hasher = |val: &_| val.hash(); // Using a simple hasher for u64 types, hash by value
    
    for x in 1..=3 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    assert!(!table.is_empty());

    let mut drained_elements = Vec::new();
    for i in table.drain() {
        drained_elements.push(i);
    }

    assert_eq!(drained_elements, vec![1, 2, 3]); // Order may vary due to hashing
    assert!(table.is_empty());
}

#[test]
fn test_drain_empty_table() {
    use hashbrown::{HashTable, DefaultHashBuilder};

    let mut table: HashTable<i32, DefaultHashBuilder> = HashTable::new_in(DefaultHashBuilder::default());
    assert!(table.is_empty());

    let drained_elements: Vec<_> = table.drain().collect();
    assert!(drained_elements.is_empty());
}

#[test]
fn test_drain_after_insert_and_clear() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new_in(DefaultHashBuilder::default());
    let hasher = |val: &_| val.hash();

    for x in 1..=5 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    assert!(!table.is_empty());

    table.clear();
    assert!(table.is_empty());

    let drained_elements: Vec<_> = table.drain().collect();
    assert!(drained_elements.is_empty());
}

