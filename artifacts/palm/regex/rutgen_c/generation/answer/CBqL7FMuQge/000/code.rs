// Answer 0

#[test]
fn test_is_empty_when_empty() {
    let sparse_set = SparseSet::new(0);
    assert!(sparse_set.is_empty());
}

#[test]
fn test_is_empty_when_not_empty() {
    let mut sparse_set = SparseSet::new(1);
    sparse_set.insert(0);
    assert!(!sparse_set.is_empty());
}

#[test]
fn test_is_empty_after_clear() {
    let mut sparse_set = SparseSet::new(1);
    sparse_set.insert(0);
    sparse_set.clear();
    assert!(sparse_set.is_empty());
}

