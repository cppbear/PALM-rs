// Answer 0

#[test]
fn test_iter_mut_basic() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new_in(Global);
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&2), 2, hasher);
    table.insert_unique(hasher(&3), 3, hasher);

    // Update all values
    for val in table.iter_mut() {
        *val *= 2;
    }

    assert_eq!(table.len(), 3);
    let mut vec: Vec<i32> = Vec::new();

    for val in &table {
        vec.push(*val);
    }

    // Sort the vector to check against expected values
    vec.sort_unstable();
    assert_eq!(vec, [2, 4, 6]);
}

#[test]
fn test_iter_mut_empty() {
    use hashbrown::HashTable;

    let mut table: HashTable<i32, Global> = HashTable::new_in(Global);

    // Iterating over an empty table should yield no results
    let mut iter = table.iter_mut();
    assert!(iter.next().is_none());
} 

#[test]
fn test_iter_mut_after_clear() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    
    let mut table = HashTable::new_in(Global);
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&2), 2, hasher);
    table.clear();

    // Iterating after clear should yield no results
    let mut iter = table.iter_mut();
    assert!(iter.next().is_none());
} 

#[test]
fn test_iter_mut_length() {
    use hashbrown::{HashTable, DefaultHashBuilder};

    let mut table = HashTable::new_in(Global);
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    for i in 1..=5 {
        table.insert_unique(hasher(&i), i, hasher);
    }

    // Ensure length is correct
    assert_eq!(table.len(), 5);

    // Update all values
    for val in table.iter_mut() {
        *val += 10;
    }

    let mut vec: Vec<i32> = table.iter().map(|&val| val).collect();
    vec.sort_unstable();
    assert_eq!(vec, [11, 12, 13, 14, 15]);
}

