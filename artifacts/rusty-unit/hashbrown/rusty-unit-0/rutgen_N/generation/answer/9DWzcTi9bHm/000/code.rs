// Answer 0

#[cfg(feature = "nightly")]
#[test]
fn test_reserve() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<i32> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    table.reserve(10, hasher);
    assert!(table.capacity() >= 10);
}

#[cfg(feature = "nightly")]
#[should_panic]
#[test]
fn test_reserve_over_max_capacity() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<i32> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    // Attempting to reserve a capacity that exceeds `isize::MAX`
    let additional_capacity = isize::MAX as usize + 1;
    table.reserve(additional_capacity, hasher);
}

