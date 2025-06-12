// Answer 0

#[test]
fn test_empty_hash_table_creation() {
    use hashbrown::HashTable;

    let table: HashTable<&str> = HashTable::new();
    assert_eq!(table.len(), 0);
    assert_eq!(table.capacity(), 0);
}

#[test]
fn test_hash_table_capacity_after_insertion() {
    use hashbrown::HashTable;

    let mut table: HashTable<&str> = HashTable::new();
    table.insert("key", "value");
    assert!(table.len() > 0);
    assert!(table.capacity() > 0);
}

