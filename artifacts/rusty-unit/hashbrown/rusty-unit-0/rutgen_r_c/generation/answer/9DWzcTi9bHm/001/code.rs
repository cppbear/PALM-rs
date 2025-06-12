// Answer 0

#[test]
fn test_reserve_basic() {
    use hashbrown::{HashTable, Global};
    use std::hash::BuildHasherDefault;
    
    let mut table: HashTable<i32, Global> = HashTable::new_in(Global);
    let hasher = |val: &i32| *val as u64;
    
    table.reserve(10, hasher);
    assert!(table.capacity() >= 10);
}

#[test]
fn test_reserve_zero_capacity() {
    use hashbrown::{HashTable, Global};

    let mut table: HashTable<i32, Global> = HashTable::new_in(Global);
    let hasher = |val: &i32| *val as u64;

    table.reserve(0, hasher);
    assert_eq!(table.capacity(), 0);
}

#[should_panic]
fn test_reserve_panic_exceeds_max_capacity() {
    use hashbrown::{HashTable, Global};

    let mut table: HashTable<u8, Global> = HashTable::new_in(Global);
    let hasher = |val: &u8| *val as u64;

    // This should panic if we assume `usize::MAX` can be legitimately used in our context
    table.reserve(usize::MAX, hasher);
}

#[test]
fn test_reserve_after_inserts() {
    use hashbrown::{HashTable, Global};

    let mut table: HashTable<i32, Global> = HashTable::new_in(Global);
    let hasher = |val: &i32| *val as u64;
    
    for i in 0..5 {
        table.insert_unique(i as u64, i, hasher);
    }

    table.reserve(10, hasher);
    assert!(table.capacity() >= 10);
    assert_eq!(table.len(), 5);
}

#[test]
fn test_reserve_and_shrink() {
    use hashbrown::{HashTable, Global};

    let mut table: HashTable<i32, Global> = HashTable::new_in(Global);
    let hasher = |val: &i32| *val as u64;

    table.reserve(20, hasher);
    assert!(table.capacity() >= 20);

    table.shrink_to(0, hasher);
    assert_eq!(table.len(), 0);
    assert!(table.capacity() >= 0);  // Check that capacity does not drop below the minimum
}

#[test]
fn test_reserve_multiple_times() {
    use hashbrown::{HashTable, Global};

    let mut table: HashTable<i32, Global> = HashTable::new_in(Global);
    let hasher = |val: &i32| *val as u64;

    table.reserve(5, hasher);
    assert!(table.capacity() >= 5);

    table.reserve(15, hasher);
    assert!(table.capacity() >= 20);
}

