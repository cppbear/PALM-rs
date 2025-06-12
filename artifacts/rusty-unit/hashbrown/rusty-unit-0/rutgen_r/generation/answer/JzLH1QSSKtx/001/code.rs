// Answer 0

#[test]
fn test_new_hash_table_capacity() {
    let table: HashTable<&str> = HashTable::new();
    assert_eq!(table.len(), 0);
    assert_eq!(table.capacity(), 0);
}

#[test]
fn test_new_hash_table_type() {
    let table: HashTable<i32> = HashTable::new();
    assert_eq!(table.len(), 0);
    assert_eq!(table.capacity(), 0);
}

#[test]
fn test_new_hash_table_empty() {
    let table: HashTable<String> = HashTable::new();
    assert_eq!(table.len(), 0);
    assert_eq!(table.capacity(), 0);
}

