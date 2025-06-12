// Answer 0

#[test]
fn test_reserve_with_positive_additional() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    
    let mut table: HashTable<i32> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    table.reserve(10, hasher);
    assert!(table.capacity() >= 10);
}

#[test]
#[should_panic]
fn test_reserve_exceeding_capacity() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    
    let mut table: HashTable<i32> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    let additional = isize::MAX as usize + 1; // This should trigger a panic
    table.reserve(additional, hasher);
}

#[test]
fn test_reserve_zero_additional() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    
    let mut table: HashTable<i32> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    table.reserve(0, hasher);
    assert_eq!(table.capacity(), 0);
}

#[test]
fn test_reserve_with_small_capacity_increase() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    
    let mut table: HashTable<i32> = HashTable::with_capacity(5);
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    table.reserve(1, hasher);
    assert!(table.capacity() >= 6);
}

#[test]
fn test_reserve_with_large_capacity() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    
    let mut table: HashTable<i32> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    table.reserve(1000, hasher);
    assert!(table.capacity() >= 1000);
}

