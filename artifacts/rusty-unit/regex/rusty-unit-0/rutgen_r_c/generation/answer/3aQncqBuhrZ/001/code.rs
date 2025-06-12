// Answer 0

#[test]
fn test_contains_existing_value() {
    let mut sparse_set = SparseSet::new(5);
    sparse_set.insert(1);
    sparse_set.insert(2);
    sparse_set.insert(3);
    assert!(sparse_set.contains(2)); // Should return true
}

#[test]
fn test_contains_non_existing_value() {
    let mut sparse_set = SparseSet::new(5);
    sparse_set.insert(1);
    sparse_set.insert(3);
    assert!(!sparse_set.contains(2)); // Should return false
}

#[test]
#[should_panic] // This will panic because the value is not inserted and will access the sparse array out of bounds
fn test_contains_out_of_bounds_access() {
    let sparse_set = SparseSet::new(2);
    let _ = sparse_set.contains(10); // Accessing an uninitialized index
}

#[test]
fn test_contains_empty_set() {
    let sparse_set = SparseSet::new(0);
    assert!(!sparse_set.contains(0)); // Should return false since the set is empty
}

#[test]
fn test_contains_from_one_element_set() {
    let mut sparse_set = SparseSet::new(1);
    sparse_set.insert(0);
    assert!(sparse_set.contains(0)); // Should return true for single element set
    assert!(!sparse_set.contains(1)); // Should return false for non-existent element
}

