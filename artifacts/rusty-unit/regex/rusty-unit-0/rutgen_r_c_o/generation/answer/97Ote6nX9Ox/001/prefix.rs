// Answer 0

#[test]
fn test_len_empty_sparse_set() {
    let sparse_set = SparseSet::new(0);
    let _ = sparse_set.len();
}

#[test]
fn test_len_large_sparse_set() {
    let sparse_set = SparseSet::new(10000);
    let _ = sparse_set.len();
}

#[test]
fn test_len_non_empty_sparse_set() {
    let mut sparse_set = SparseSet::new(5);
    sparse_set.insert(1);
    sparse_set.insert(2);
    let _ = sparse_set.len();
}

#[test]
fn test_len_after_clearing_sparse_set() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.insert(3);
    sparse_set.insert(4);
    sparse_set.clear();
    let _ = sparse_set.len();
}

#[test]
fn test_len_with_various_sizes() {
    for size in [1, 50, 100, 9999, 10000].iter() {
        let sparse_set = SparseSet::new(*size);
        let _ = sparse_set.len();
    }
}

