// Answer 0

#[test]
fn test_capacity_empty_spars_set() {
    let sparse_set = SparseSet::new(0);
    let result = sparse_set.capacity();
}

#[test]
fn test_capacity_non_empty_sparse_set() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.insert(0);
    sparse_set.insert(5);
    sparse_set.insert(9);
    let result = sparse_set.capacity();
}

#[test]
fn test_capacity_full_sparse_set() {
    let mut sparse_set = SparseSet::new(100);
    for i in 0..100 {
        sparse_set.insert(i);
    }
    let result = sparse_set.capacity();
}

#[test]
fn test_capacity_with_large_size() {
    let mut sparse_set = SparseSet::new(1000);
    for i in 0..1000 {
        sparse_set.insert(i);
    }
    let result = sparse_set.capacity();
}

#[test]
fn test_capacity_empty_after_clear() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.insert(1);
    sparse_set.clear();
    let result = sparse_set.capacity();
}

#[test]
fn test_capacity_insert_lower_bound() {
    let mut sparse_set = SparseSet::new(1);
    sparse_set.insert(0);
    let result = sparse_set.capacity();
}

#[test]
fn test_capacity_insert_upper_bound() {
    let mut sparse_set = SparseSet::new(1000);
    sparse_set.insert(999);
    let result = sparse_set.capacity();
}

