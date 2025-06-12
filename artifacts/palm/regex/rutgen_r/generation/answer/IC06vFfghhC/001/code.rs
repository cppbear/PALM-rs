// Answer 0

#[test]
fn test_new_sparse_set_zero_size() {
    let sparse_set = new(0);
    assert_eq!(sparse_set.dense.len(), 0);
    assert_eq!(sparse_set.sparse.len(), 0);
    assert_eq!(sparse_set.size, 0);
}

#[test]
fn test_new_sparse_set_small_size() {
    let sparse_set = new(5);
    assert_eq!(sparse_set.dense.len(), 5);
    assert_eq!(sparse_set.sparse.len(), 5);
    assert_eq!(sparse_set.dense, vec![0, 0, 0, 0, 0]);
    assert_eq!(sparse_set.sparse, vec![0, 0, 0, 0, 0]);
    assert_eq!(sparse_set.size, 0);
}

#[test]
fn test_new_sparse_set_large_size() {
    let sparse_set = new(1000);
    assert_eq!(sparse_set.dense.len(), 1000);
    assert_eq!(sparse_set.sparse.len(), 1000);
    assert_eq!(sparse_set.dense, vec![0; 1000]);
    assert_eq!(sparse_set.sparse, vec![0; 1000]);
    assert_eq!(sparse_set.size, 0);
}

