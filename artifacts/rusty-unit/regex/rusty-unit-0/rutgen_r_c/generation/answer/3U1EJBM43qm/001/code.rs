// Answer 0

#[test]
fn test_clear_on_non_empty_sparse_set() {
    let mut sparse_set = SparseSet {
        dense: vec![1, 2, 3],
        sparse: vec![0, 1, 2],
        size: 3,
    };
    
    sparse_set.clear();
    
    assert_eq!(sparse_set.size, 0);
    assert!(sparse_set.is_empty());
}

#[test]
fn test_clear_on_empty_sparse_set() {
    let mut sparse_set = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };
    
    sparse_set.clear();
    
    assert_eq!(sparse_set.size, 0);
    assert!(sparse_set.is_empty());
}

#[test]
fn test_clear_multiple_calls() {
    let mut sparse_set = SparseSet {
        dense: vec![1, 2, 3, 4, 5],
        sparse: vec![0, 1, 2, 3, 4],
        size: 5,
    };
    
    sparse_set.clear();
    assert_eq!(sparse_set.size, 0);
    
    sparse_set.clear(); // Second call should also maintain size 0
    assert_eq!(sparse_set.size, 0);
    assert!(sparse_set.is_empty());
}

