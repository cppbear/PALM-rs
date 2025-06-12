// Answer 0

#[test]
fn test_with_capacity_zero() {
    let table: HashTable<&str> = HashTable::with_capacity(0);
    assert_eq!(table.raw.table, RawTableInner::NEW);
    assert!(table.raw.alloc.capacity() == 0);
}

#[test]
fn test_with_capacity_positive() {
    let capacity = 10;
    let table: HashTable<&str> = HashTable::with_capacity(capacity);
    assert_eq!(table.raw.table, RawTableInner::NEW);
    assert!(table.raw.alloc.capacity() >= capacity);
}

#[test]
fn test_with_capacity_large() {
    let capacity = usize::MAX;
    let table: HashTable<&str> = HashTable::with_capacity(capacity);
    assert_eq!(table.raw.table, RawTableInner::NEW);
    assert!(table.raw.alloc.capacity() >= capacity);
}

