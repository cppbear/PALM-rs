// Answer 0

#[test]
fn test_sparse_set_len_empty() {
    let sparse_set = SparseSet::new(10);
    assert_eq!(sparse_set.len(), 0);
}

#[test]
fn test_sparse_set_len_after_insert() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.insert(5);
    assert_eq!(sparse_set.len(), 1);
    sparse_set.insert(1);
    assert_eq!(sparse_set.len(), 2);
}

#[test]
fn test_sparse_set_len_after_clear() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.insert(3);
    sparse_set.insert(7);
    assert_eq!(sparse_set.len(), 2);
    sparse_set.clear();
    assert_eq!(sparse_set.len(), 0);
}

#[test]
fn test_sparse_set_len_large() {
    let mut sparse_set = SparseSet::new(100);
    for i in 0..50 {
        sparse_set.insert(i);
    }
    assert_eq!(sparse_set.len(), 50);
}

