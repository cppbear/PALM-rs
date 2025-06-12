// Answer 0

#[test]
fn test_clear_non_empty_sparse_set() {
    let mut sparse_set = SparseSet {
        dense: vec![1, 2, 3],
        sparse: vec![0, 1, 2],
        size: 3,
    };
    sparse_set.clear();
    assert_eq!(sparse_set.len(), 0);
    assert!(sparse_set.is_empty());
}

#[test]
fn test_clear_empty_sparse_set() {
    let mut sparse_set = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };
    sparse_set.clear();
    assert_eq!(sparse_set.len(), 0);
    assert!(sparse_set.is_empty());
}

#[test]
fn test_clear_after_insertion() {
    let mut sparse_set = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };
    sparse_set.insert(10);
    sparse_set.insert(20);
    sparse_set.clear();
    assert_eq!(sparse_set.len(), 0);
    assert!(sparse_set.is_empty());
}

