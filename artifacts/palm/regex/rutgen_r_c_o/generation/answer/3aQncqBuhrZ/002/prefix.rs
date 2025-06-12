// Answer 0

#[test]
fn test_contains_empty() {
    let sparse_set = SparseSet::new(0);
    let value = 0;
    let result = sparse_set.contains(value);
}

#[test]
fn test_contains_non_existing_value() {
    let mut sparse_set = SparseSet::new(5);
    let value = 10;
    sparse_set.insert(1);
    let result = sparse_set.contains(value);
}

#[test]
fn test_contains_existing_value() {
    let mut sparse_set = SparseSet::new(5);
    let value = 2;
    sparse_set.insert(value);
    let result = sparse_set.contains(value);
}

#[test]
#[should_panic]
fn test_contains_out_of_bounds_index() {
    let mut sparse_set = SparseSet::new(3);
    let value = 5;
    let result = sparse_set.contains(value);
}

#[test]
fn test_contains_when_sparse_cleared() {
    let mut sparse_set = SparseSet::new(5);
    let value = 1;
    sparse_set.insert(value);
    sparse_set.clear();
    let result = sparse_set.contains(value);
}

#[test]
fn test_contains_with_maximum_size() {
    let mut sparse_set = SparseSet::new(5);
    let value = 4;
    sparse_set.insert(value);
    let result = sparse_set.contains(value);
}

#[test]
fn test_contains_when_sparse_full() {
    let mut sparse_set = SparseSet::new(3);
    sparse_set.insert(0);
    sparse_set.insert(1);
    
    let value = 2;
    let result = sparse_set.contains(value);
}

