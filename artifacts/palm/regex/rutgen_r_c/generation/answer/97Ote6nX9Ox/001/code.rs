// Answer 0

#[test]
fn test_sparse_set_len_non_empty() {
    let sparse_set = SparseSet::new(5);
    sparse_set.insert(0);
    sparse_set.insert(1);
    sparse_set.insert(2);
    assert_eq!(sparse_set.len(), 3);
}

#[test]
fn test_sparse_set_len_empty() {
    let sparse_set = SparseSet::new(0);
    assert_eq!(sparse_set.len(), 0);
}

#[test]
fn test_sparse_set_len_large() {
    let sparse_set = SparseSet::new(100);
    for i in 0..100 {
        sparse_set.insert(i);
    }
    assert_eq!(sparse_set.len(), 100);
}

#[test]
fn test_sparse_set_len_after_clear() {
    let mut sparse_set = SparseSet::new(5);
    sparse_set.insert(0);
    sparse_set.insert(1);
    sparse_set.clear();
    assert_eq!(sparse_set.len(), 0);
}

