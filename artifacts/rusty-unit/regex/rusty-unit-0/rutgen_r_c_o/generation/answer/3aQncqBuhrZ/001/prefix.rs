// Answer 0

#[test]
fn test_contains_valid_case() {
    let mut set = SparseSet::new(5);
    set.insert(0);
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);
    assert!(set.contains(0));
}

#[test]
fn test_contains_another_valid_case() {
    let mut set = SparseSet::new(5);
    set.insert(2);
    set.insert(4);
    set.insert(6);
    assert!(set.contains(4));
}

#[test]
fn test_contains_boundary_value() {
    let mut set = SparseSet::new(3);
    set.insert(0);
    set.insert(1);
    assert!(set.contains(1));
}

#[test]
fn test_contains_edge_case_empty() {
    let set = SparseSet::new(3);
    assert!(!set.contains(1));
}

#[test]
#[should_panic]
fn test_contains_panic_value_out_of_bounds() {
    let mut set = SparseSet::new(3);
    set.insert(1);
    set.contains(5);
}

#[test]
#[should_panic]
fn test_contains_panic_sparse_index_out_of_bounds() {
    let mut set = SparseSet::new(3);
    set.insert(1);
    set.sparse.push(3); // Manually manipulating sparse to cause panic
    assert!(set.contains(1));
}

