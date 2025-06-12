// Answer 0

#[test]
fn test_shrink_to_valid_cases() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::with_capacity_in(100, DefaultHashBuilder::default());
    let hasher = |val: &u32| val.clone() as u64;

    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&2), 2, hasher);
    
    assert!(table.capacity() >= 100);
    
    table.shrink_to(10, hasher);
    assert!(table.capacity() >= 10);
    
    table.shrink_to(0, hasher);
    assert!(table.capacity() >= 2);
}

#[should_panic]
#[test]
fn test_shrink_to_with_min_capacity_greater_than_current_capacity() {
    let mut table = HashTable::with_capacity_in(5, DefaultHashBuilder::default());
    let hasher = |val: &u32| val.clone() as u64;

    table.insert_unique(hasher(&1), 1, hasher);
    
    // Force a panic by trying to shrink below the current capacity
    table.shrink_to(10, hasher);
}

#[should_panic]
#[test]
fn test_shrink_to_with_zero_min_capacity_on_empty_table() {
    let mut table = HashTable::with_capacity_in(0, DefaultHashBuilder::default());
    let hasher = |val: &u32| val.clone() as u64;

    // Force a panic as there are no elements and attempting to set min capacity
    table.shrink_to(1, hasher);
}

