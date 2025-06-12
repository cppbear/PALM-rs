// Answer 0

#[test]
fn test_threads_new() {
    let threads = Threads::new();
    assert_eq!(threads.slots_per_thread, 0);
    assert!(threads.caps.is_empty());
    assert_eq!(threads.set.len(), 0);
    assert!(threads.set.is_empty());
}

#[test]
fn test_sparse_set_new() {
    let sparse_set = SparseSet::new(0);
    assert_eq!(sparse_set.size, 0);
    assert!(sparse_set.dense.is_empty());
    assert!(sparse_set.sparse.is_empty());
}

#[test]
fn test_sparse_set_insert_and_contains() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.insert(5);
    assert_eq!(sparse_set.len(), 1);
    assert!(sparse_set.contains(5));
    assert!(!sparse_set.contains(1));
}

#[test]
fn test_sparse_set_clear() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.insert(5);
    sparse_set.clear();
    assert!(sparse_set.is_empty());
}

