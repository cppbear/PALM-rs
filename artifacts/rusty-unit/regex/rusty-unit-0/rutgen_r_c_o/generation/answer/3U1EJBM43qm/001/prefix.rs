// Answer 0

#[test]
fn test_clear_empty_sparse_set() {
    let mut set = SparseSet::new(10);
    set.clear();
}

#[test]
fn test_clear_non_empty_sparse_set() {
    let mut set = SparseSet::new(10);
    set.insert(1);
    set.insert(2);
    set.clear();
}

#[test]
fn test_clear_full_sparse_set() {
    let mut set = SparseSet::new(100);
    for i in 0..100 {
        set.insert(i);
    }
    set.clear();
}

#[test]
fn test_clear_large_sparse_set() {
    let mut set = SparseSet::new(10000);
    for i in 0..10000 {
        set.insert(i);
    }
    set.clear();
}

#[test]
fn test_clear_sparse_set_with_repeated_insertions() {
    let mut set = SparseSet::new(50);
    set.insert(10);
    set.insert(10);
    set.clear();
}

