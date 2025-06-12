// Answer 0

#[test]
fn test_insert_with_valid_value() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.insert(3);
    assert_eq!(sparse_set.len(), 1);
    assert!(sparse_set.contains(3));
}

#[test]
#[should_panic]
fn test_insert_out_of_bounds_dense() {
    let mut sparse_set = SparseSet::new(1);
    sparse_set.insert(2);
}

#[test]
#[should_panic]
fn test_insert_invalid_sparse() {
    let mut sparse_set = SparseSet::new(1);
    sparse_set.insert(1);
}

#[test]
fn test_insert_at_capacity() {
    let mut sparse_set = SparseSet::new(3);
    sparse_set.insert(0);
    sparse_set.insert(1);
    sparse_set.insert(2);
    assert_eq!(sparse_set.len(), 3);
    assert!(sparse_set.contains(0));
    assert!(sparse_set.contains(1));
    assert!(sparse_set.contains(2));
}

#[test]
fn test_insert_with_clearing() {
    let mut sparse_set = SparseSet::new(5);
    sparse_set.insert(1);
    sparse_set.clear();
    assert!(sparse_set.is_empty());
    sparse_set.insert(1);
    assert_eq!(sparse_set.len(), 1);
    assert!(sparse_set.contains(1));
}

