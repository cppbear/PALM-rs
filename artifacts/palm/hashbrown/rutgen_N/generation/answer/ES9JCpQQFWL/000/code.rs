// Answer 0

#[test]
fn test_with_capacity_zero() {
    let table = hashbrown::raw::with_capacity(0);
    assert!(table.is_empty());
}

#[test]
fn test_with_capacity_non_zero() {
    let capacity = 10;
    let table = hashbrown::raw::with_capacity(capacity);
    assert!(table.capacity() >= capacity);
}

#[test]
fn test_with_capacity_edge_case() {
    let capacity = std::usize::MAX;
    let table = hashbrown::raw::with_capacity(capacity);
    assert!(table.capacity() >= capacity);
}

