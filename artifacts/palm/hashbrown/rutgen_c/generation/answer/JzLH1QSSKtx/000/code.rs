// Answer 0

#[test]
fn test_hash_table_new() {
    let table: HashTable<&str> = HashTable::new();
    assert_eq!(table.len(), 0);
    assert_eq!(table.raw.table.capacity(), 0);
}

#[test]
fn test_hash_table_with_capacity_zero() {
    let table = HashTable::<&str>::with_capacity(0);
    assert_eq!(table.len(), 0);
    assert_eq!(table.raw.table.capacity(), 0);
}

#[test]
fn test_hash_table_with_non_zero_capacity() {
    let capacity = 10;
    let table = HashTable::<&str>::with_capacity(capacity);
    assert_eq!(table.len(), 0);
    assert!(table.raw.table.capacity() >= capacity);
}

