// Answer 0

#[test]
fn test_raw_table_with_capacity_zero() {
    let table: RawTable<i32> = RawTable::with_capacity(0);
    assert_eq!(table.len(), 0);
    assert!(table.is_empty());
}

#[test]
fn test_raw_table_with_capacity_one() {
    let table: RawTable<i32> = RawTable::with_capacity(1);
    assert_eq!(table.len(), 0);
    assert!(table.is_empty());
}

#[test]
fn test_raw_table_with_large_capacity() {
    let capacity = 1024;
    let table: RawTable<i32> = RawTable::with_capacity(capacity);
    assert_eq!(table.len(), 0);
    assert!(table.is_empty());
}

