// Answer 0

#[test]
fn test_new_zero_size() {
    let sparse_set = SparseSet::new(0);
}

#[test]
fn test_new_small_size() {
    let sparse_set = SparseSet::new(1);
}

#[test]
fn test_new_size_five() {
    let sparse_set = SparseSet::new(5);
}

#[test]
fn test_new_size_ten() {
    let sparse_set = SparseSet::new(10);
}

#[test]
fn test_new_large_size() {
    let sparse_set = SparseSet::new(1_000_000);
}

#[test]
fn test_new_edge_case_size() {
    let sparse_set = SparseSet::new(10_000);
}

