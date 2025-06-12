// Answer 0

#[test]
fn test_with_capacity_non_zero() {
    let capacity = 10;
    let table: HashTable<&str> = HashTable::with_capacity(capacity);
    assert_eq!(table.len(), 0);
    assert!(table.capacity() >= capacity);
}

#[test]
fn test_with_capacity_zero() {
    let capacity = 0;
    let table: HashTable<&str> = HashTable::with_capacity(capacity);
    assert_eq!(table.len(), 0);
    assert_eq!(table.capacity(), 0);
}

#[test]
fn test_with_capacity_boundary() {
    let capacity = usize::MAX;
    let table: HashTable<&str> = HashTable::with_capacity(capacity);
    assert_eq!(table.len(), 0);
    assert!(table.capacity() >= capacity);
}

