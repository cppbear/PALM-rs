// Answer 0

#[test]
fn test_with_capacity_zero() {
    let capacity = 0;
    let table = hashbrown::HashMap::with_capacity(capacity);
    assert_eq!(table.len(), 0);
}

#[test]
fn test_with_capacity_small_number() {
    let capacity = 5;
    let table = hashbrown::HashMap::with_capacity(capacity);
    assert!(table.capacity() >= capacity);
}

#[test]
fn test_with_capacity_large_number() {
    let capacity = 1_000_000;
    let table = hashbrown::HashMap::with_capacity(capacity);
    assert!(table.capacity() >= capacity);
}

#[test]
#[should_panic]
fn test_with_capacity_negative() {
    let capacity = usize::MAX; // Assuming this triggers an allocation issue
    let _table = hashbrown::HashMap::with_capacity(capacity);
}

