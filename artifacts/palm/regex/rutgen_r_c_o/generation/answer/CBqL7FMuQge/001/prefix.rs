// Answer 0

#[test]
fn test_sparse_set_is_empty_with_zero_size() {
    let sparse_set = SparseSet::new(0);
    sparse_set.is_empty();
}

#[test]
fn test_sparse_set_is_empty_after_inserting_and_removing() {
    let mut sparse_set = SparseSet::new(1);
    sparse_set.insert(0);
    sparse_set.clear();
    sparse_set.is_empty();
}

#[test]
fn test_sparse_set_is_empty_with_non_zero_size() {
    let mut sparse_set = SparseSet::new(1);
    sparse_set.insert(1);
    sparse_set.is_empty();  
}

