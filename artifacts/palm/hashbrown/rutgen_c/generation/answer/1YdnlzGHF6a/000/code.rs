// Answer 0

#[test]
fn test_capacity_empty() {
    let table: HashTable<i32> = HashTable::new_in(Global);
    assert_eq!(table.capacity(), 0);
}

#[test]
fn test_capacity_with_capacity() {
    let table: HashTable<i32> = HashTable::with_capacity_in(100, Global);
    assert!(table.capacity() >= 100);
}

#[test]
fn test_capacity_after_insert() {
    let mut table: HashTable<i32> = HashTable::with_capacity_in(10, Global);
    assert_eq!(table.capacity(), 10);
    // Insert an element
    table.insert_unique(1, 42, |x| *x);
    assert!(table.capacity() >= 10); // Ensure capacity remains at least 10
}

#[test]
fn test_capacity_shrink() {
    let mut table: HashTable<i32> = HashTable::with_capacity_in(100, Global);
    assert!(table.capacity() >= 100);
    // Simulate shrink to fit which should not affect capacity in current implementation
    table.shrink_to_fit(|x| *x);
    assert!(table.capacity() >= 100);
}

