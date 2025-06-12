// Answer 0

#[test]
fn test_iter_hash_mut() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    table.insert_unique(hasher(&1), 2, hasher);
    table.insert_unique(hasher(&1), 3, hasher);
    table.insert_unique(hasher(&2), 5, hasher);

    // Update matching values
    for val in table.iter_hash_mut(hasher(&1)) {
        *val *= 2;
    }

    assert_eq!(table.len(), 3);
    let mut vec: Vec<i32> = Vec::new();

    for val in &table {
        vec.push(*val);
    }

    // The values will contain 4 and 6 and may contain either 5 or 10.
    assert!(vec.contains(&4));
    assert!(vec.contains(&6));

    assert_eq!(table.len(), 3);
}

#[test]
fn test_iter_hash_mut_empty() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Try to iterate over a hash that has no elements
    let mut count = 0;
    for _ in table.iter_hash_mut(hasher(&999)) {
        count += 1;
    }

    assert_eq!(count, 0);
}

#[should_panic]
fn test_iter_hash_mut_invalid_access() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    table.insert_unique(hasher(&1), 2, hasher);

    // This line should panic if attempting to mutate the iterator concurrently
    for val in table.iter_hash_mut(hasher(&1)) {
        *val += 1;
        // Simulate mutable action that alters the structure (invalid usage)
        table.insert_unique(hasher(&3), 3, hasher);
    }
}

