// Answer 0

#[test]
fn test_contains_existing_element() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.insert(5);
    assert!(sparse_set.contains(5));
}

#[test]
fn test_contains_non_existing_element() {
    let sparse_set = SparseSet::new(10);
    assert!(!sparse_set.contains(5));
}

#[test]
fn test_contains_boundary_element() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.insert(9);
    assert!(sparse_set.contains(9));
    assert!(!sparse_set.contains(10)); // Out of bounds
}

#[test]
fn test_contains_empty_set() {
    let sparse_set = SparseSet::new(0);
    assert!(!sparse_set.contains(0));
}

