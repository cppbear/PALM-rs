// Answer 0

#[test]
fn test_capacity_with_large_initial_capacity() {
    let table: hashbrown::HashTable<i32> = hashbrown::HashTable::with_capacity(100);
    assert!(table.capacity() >= 100);
}

#[test]
fn test_capacity_with_small_initial_capacity() {
    let table: hashbrown::HashTable<i32> = hashbrown::HashTable::with_capacity(0);
    assert!(table.capacity() >= 0);
}

#[test]
fn test_capacity_after_insertions() {
    let mut table: hashbrown::HashTable<i32> = hashbrown::HashTable::with_capacity(10);
    for i in 0..10 {
        table.insert(i, i);
    }
    assert!(table.capacity() >= 10);
}

#[test]
fn test_capacity_no_reallocation() {
    let mut table: hashbrown::HashTable<i32> = hashbrown::HashTable::with_capacity(5);
    assert_eq!(table.capacity(), 5);
    for i in 0..5 {
        table.insert(i, i);
    }
    assert_eq!(table.capacity(), 5);
}

#[test]
fn test_capacity_with_zero_elements() {
    let table: hashbrown::HashTable<i32> = hashbrown::HashTable::with_capacity(0);
    assert_eq!(table.capacity(), 0);
}

#[test]
fn test_capacity_of_empty_table() {
    let table: hashbrown::HashTable<i32> = hashbrown::HashTable::with_capacity(20);
    assert!(table.capacity() >= 20);
}

