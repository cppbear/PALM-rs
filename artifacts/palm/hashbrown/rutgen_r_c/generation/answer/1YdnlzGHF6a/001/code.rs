// Answer 0

#[test]
fn test_capacity_with_default_allocator() {
    use crate::hashbrown::HashTable;

    let table: HashTable<i32> = HashTable::new_in(Global);
    assert_eq!(table.capacity(), 0);
}

#[test]
fn test_capacity_with_specific_size() {
    use crate::hashbrown::HashTable;

    let table: HashTable<i32> = HashTable::with_capacity_in(50, Global);
    assert!(table.capacity() >= 50);
}

#[test]
fn test_capacity_after_insertions() {
    use crate::hashbrown::HashTable;

    let mut table: HashTable<i32> = HashTable::with_capacity_in(10, Global);
    for i in 1..=10 {
        table.insert_unique(i as u64, i, |&x| x);
    }
    assert!(table.capacity() >= 10);
}

#[test]
fn test_capacity_after_clear() {
    use crate::hashbrown::HashTable;

    let mut table: HashTable<i32> = HashTable::with_capacity_in(100, Global);
    for i in 1..=100 {
        table.insert_unique(i as u64, i, |&x| x);
    }
    table.clear();
    assert_eq!(table.capacity(), 100); // Capacity should remain the same after clear
}

#[test]
fn test_capacity_with_no_elements() {
    use crate::hashbrown::HashTable;

    let table: HashTable<i32> = HashTable::new_in(Global);
    assert_eq!(table.capacity(), 0);
}

#[test]
fn test_capacity_large_size() {
    use crate::hashbrown::HashTable;

    let large_capacity = 1_000_000;
    let table: HashTable<i32> = HashTable::with_capacity_in(large_capacity, Global);
    assert!(table.capacity() >= large_capacity);
}

