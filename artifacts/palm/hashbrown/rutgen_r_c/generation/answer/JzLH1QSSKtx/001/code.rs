// Answer 0

#[test]
fn test_hash_table_new() {
    let table: HashTable<&str> = HashTable::new();
    assert_eq!(table.raw.table, RawTableInner::NEW);
}

#[test]
fn test_hash_table_new_capacity_zero() {
    let table: HashTable<i32> = HashTable::new();
    assert_eq!(table.raw.table, RawTableInner::NEW);
}

