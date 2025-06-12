// Answer 0

#[test]
fn test_contains_valid_element() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.insert(5);
    assert!(sparse_set.contains(5));
}

#[test]
fn test_contains_invalid_element() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.insert(3);
    assert!(!sparse_set.contains(5));
}

#[test]
#[should_panic]
fn test_contains_out_of_bounds_access() {
    let sparse_set = SparseSet::new(10);
    sparse_set.contains(15); // This should panic as 15 is out of bounds
}

#[test]
fn test_contains_empty_set() {
    let sparse_set = SparseSet::new(10);
    assert!(!sparse_set.contains(0)); // Checks for value in an empty set
}

#[test]
fn test_contains_element_not_in_set() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.insert(1);
    sparse_set.insert(2);
    assert!(!sparse_set.contains(3)); // Checks for non-existent element
}

