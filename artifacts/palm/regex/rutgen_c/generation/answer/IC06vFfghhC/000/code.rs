// Answer 0

#[test]
fn test_sparse_set_new_zero_size() {
    let sparse_set = SparseSet::new(0);
    assert_eq!(sparse_set.dense.len(), 0);
    assert_eq!(sparse_set.sparse.len(), 0);
    assert_eq!(sparse_set.size, 0);
}

#[test]
fn test_sparse_set_new_non_zero_size() {
    let sparse_set = SparseSet::new(10);
    assert_eq!(sparse_set.dense.len(), 10);
    assert_eq!(sparse_set.sparse.len(), 10);
    assert_eq!(sparse_set.size, 0);
}

