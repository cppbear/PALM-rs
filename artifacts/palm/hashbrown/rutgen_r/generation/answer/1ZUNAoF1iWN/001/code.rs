// Answer 0

#[test]
fn test_shrink_to_valid_case() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::with_capacity(100);
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&2), 2, hasher);
    assert!(table.capacity() >= 100);
    
    table.shrink_to(10, hasher);
    assert!(table.capacity() >= 10);
    
    table.shrink_to(0, hasher);
    assert!(table.capacity() >= 2);
}

#[should_panic(expected = "current capacity is smaller than the supplied minimum capacity")]
#[test]
fn test_shrink_to_panic_case() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::with_capacity(5);
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&2), 2, hasher);
    assert!(table.capacity() >= 5);
    
    table.shrink_to(10, hasher); // This should panic since current capacity is 5
}

