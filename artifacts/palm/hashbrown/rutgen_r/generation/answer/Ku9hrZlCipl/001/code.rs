// Answer 0

#[test]
fn test_shrink_to_fit_with_empty_table() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<u64, u64> = HashTable::with_capacity(10);
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    table.shrink_to_fit(hasher);
    assert_eq!(table.capacity(), 10); // Should remain unchanged since it's empty
}

#[test]
fn test_shrink_to_fit_with_single_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<u64, u64> = HashTable::with_capacity(10);
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    table.insert_unique(hasher(&1), 1, hasher);
    assert!(table.capacity() >= 10);
    table.shrink_to_fit(hasher);
    assert!(table.capacity() >= 1); // Capacity should reduce to fit one entry
}

#[test]
fn test_shrink_to_fit_with_multiple_entries() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<u64, u64> = HashTable::with_capacity(20);
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    for i in 1..=5 {
        table.insert_unique(hasher(&i), i, hasher);
    }
    assert!(table.capacity() >= 20);
    
    table.shrink_to_fit(hasher);
    assert!(table.capacity() >= 5); // Should fit all 5 entries
}

#[test]
fn test_shrink_to_fit_with_full_capacity() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<u64, u64> = HashTable::with_capacity(5);
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    for i in 1..=5 {
        table.insert_unique(hasher(&i), i, hasher);
    }
    assert!(table.capacity() >= 5);
    
    table.shrink_to_fit(hasher);
    assert_eq!(table.capacity(), 5); // Capacity should remain the same since it's full
}

#[test]
#[should_panic]
fn test_shrink_to_fit_with_invalid_hasher() {
    use hashbrown::{HashTable};
    
    let mut table: HashTable<u64, u64> = HashTable::with_capacity(10);
    
    // Attempting to use a hasher that does not yield a consistent hash value
    table.shrink_to_fit(|val: &u64| {
        // This could potentially lead to inconsistent hashing
        if *val == 0 {
            0 // Invalid case as zero value won't match inserted values
        } else {
            *val // Valid case
        }
    });
}

