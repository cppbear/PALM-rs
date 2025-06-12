// Answer 0

#[test]
fn test_insert() {
    let mut sparse_set = SparseSet::new(10);
    assert!(sparse_set.is_empty());
    assert_eq!(sparse_set.len(), 0);
    
    sparse_set.insert(5);
    assert_eq!(sparse_set.len(), 1);
    assert!(sparse_set.contains(5));
    assert!(!sparse_set.is_empty());
    
    sparse_set.insert(3);
    assert_eq!(sparse_set.len(), 2);
    assert!(sparse_set.contains(3));
}

#[test]
fn test_insert_boundary() {
    let mut sparse_set = SparseSet::new(1);
    
    sparse_set.insert(0);
    assert_eq!(sparse_set.len(), 1);
    assert!(sparse_set.contains(0));

    // Uncommenting the following line should panic if the implementation checks for bounds
    // sparse_set.insert(1);
}

#[test]
fn test_insert_exceeding_capacity() {
    let mut sparse_set = SparseSet::new(2);
    sparse_set.insert(0);
    sparse_set.insert(1);
    
    assert_eq!(sparse_set.len(), 2);

    // Uncommenting the following line should panic if the implementation checks for bounds
    // sparse_set.insert(2);
}

