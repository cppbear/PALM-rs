// Answer 0

#[test]
fn test_sparse_set_new_zero_size() {
    let set = SparseSet::new(0);
    assert_eq!(set.dense, Vec::<usize>::new());
    assert_eq!(set.sparse, Vec::<usize>::new());
    assert_eq!(set.size, 0);
}

#[test]
fn test_sparse_set_new_small_size() {
    let set = SparseSet::new(1);
    assert_eq!(set.dense, vec![0; 1]);
    assert_eq!(set.sparse, vec![0; 1]);
    assert_eq!(set.size, 0);
}

#[test]
fn test_sparse_set_new_large_size() {
    let set = SparseSet::new(100);
    assert_eq!(set.dense, vec![0; 100]);
    assert_eq!(set.sparse, vec![0; 100]);
    assert_eq!(set.size, 0);
}

