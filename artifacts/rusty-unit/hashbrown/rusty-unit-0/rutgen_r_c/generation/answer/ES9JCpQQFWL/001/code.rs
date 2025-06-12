// Answer 0

#[test]
fn test_with_capacity_zero() {
    let table: RawTable<u8> = RawTable::with_capacity(0);
    assert_eq!(table.len(), 0);
    assert_eq!(table.capacity(), 0);
}

#[test]
fn test_with_capacity_one() {
    let table: RawTable<u8> = RawTable::with_capacity(1);
    assert_eq!(table.len(), 0);
    assert!(table.capacity() >= 1);
}

#[test]
fn test_with_capacity_large() {
    let capacity = usize::MAX;
    let table: RawTable<u8> = RawTable::with_capacity(capacity);
    assert_eq!(table.len(), 0);
    assert!(table.capacity() >= capacity);
}

#[test]
fn test_with_capacity_small_non_zero() {
    let table: RawTable<u8> = RawTable::with_capacity(5);
    assert_eq!(table.len(), 0);
    assert!(table.capacity() >= 5);
}

