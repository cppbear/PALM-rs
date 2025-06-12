// Answer 0

#[test]
fn test_sparse_set_is_empty_when_size_zero() {
    let sparse_set = SparseSet::new(0);
    assert!(sparse_set.is_empty());
}

#[test]
fn test_sparse_set_is_empty_when_size_non_zero() {
    let mut sparse_set = SparseSet::new(5);
    sparse_set.insert(1);
    assert!(!sparse_set.is_empty());
}

#[test]
fn test_sparse_set_is_empty_after_clear() {
    let mut sparse_set = SparseSet::new(5);
    sparse_set.insert(1);
    sparse_set.clear();
    assert!(sparse_set.is_empty());
}

