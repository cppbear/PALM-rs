// Answer 0

#[test]
fn test_sparse_set_capacity_empty() {
    let sparse_set = SparseSet::new(0);
    assert_eq!(sparse_set.capacity(), 0);
}

#[test]
fn test_sparse_set_capacity_non_empty() {
    let mut sparse_set = SparseSet::new(5);
    assert_eq!(sparse_set.capacity(), 0);
    sparse_set.insert(0);
    sparse_set.insert(1);
    assert_eq!(sparse_set.capacity(), 2);
}

#[test]
fn test_sparse_set_capacity_after_clearing() {
    let mut sparse_set = SparseSet::new(3);
    sparse_set.insert(0);
    sparse_set.insert(1);
    sparse_set.clear();
    assert_eq!(sparse_set.capacity(), 2);
}

#[test]
fn test_sparse_set_capacity_after_insertions() {
    let mut sparse_set = SparseSet::new(10);
    for i in 0..10 {
        sparse_set.insert(i);
    }
    assert_eq!(sparse_set.capacity(), 10);
}

