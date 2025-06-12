// Answer 0

#[test]
fn test_shrink_to_fit() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::with_capacity(100);
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&2), 2, hasher);
    
    assert!(table.capacity() >= 100);
    table.shrink_to_fit(hasher);
    assert!(table.capacity() >= 2);
}

#[test]
fn test_shrink_to_fit_empty() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<u64, u64> = HashTable::with_capacity(10);
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    assert_eq!(table.capacity(), 10);
    table.shrink_to_fit(hasher);
    assert_eq!(table.capacity(), 0);
} 

#[test]
fn test_shrink_to_fit_single_element() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::with_capacity(10);
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    table.insert_unique(hasher(&1), 1, hasher);
    
    assert!(table.capacity() >= 10);
    table.shrink_to_fit(hasher);
    assert!(table.capacity() >= 1);
}

