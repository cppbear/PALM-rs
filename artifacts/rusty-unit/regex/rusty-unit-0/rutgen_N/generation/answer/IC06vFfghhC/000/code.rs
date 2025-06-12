// Answer 0

#[test]
fn test_sparse_set_new() {
    let size = 10;
    let sparse_set = new(size);
    assert_eq!(sparse_set.dense.len(), size);
    assert_eq!(sparse_set.sparse.len(), size);
    assert_eq!(sparse_set.size, 0);
}

#[test]
fn test_sparse_set_new_zero_size() {
    let size = 0;
    let sparse_set = new(size);
    assert_eq!(sparse_set.dense.len(), size);
    assert_eq!(sparse_set.sparse.len(), size);
    assert_eq!(sparse_set.size, 0);
}

