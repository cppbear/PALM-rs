// Answer 0

#[test]
fn test_is_disjoint_equal_length_disjoint_sets() {
    use indexmap::IndexSet;
    use std::collections::hash_map::RandomState;

    let set_a: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![1, 2, 3]);
    let set_b: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![4, 5, 6]);
    
    assert!(set_a.is_disjoint(&set_b));
}

#[test]
fn test_is_disjoint_equal_length_non_disjoint_sets() {
    use indexmap::IndexSet;
    use std::collections::hash_map::RandomState;

    let set_a: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![1, 2, 3]);
    let set_b: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![3, 4, 5]);
    
    assert!(!set_a.is_disjoint(&set_b));
}

#[test]
fn test_is_disjoint_equal_length_empty_and_non_empty_sets() {
    use indexmap::IndexSet;
    use std::collections::hash_map::RandomState;

    let set_a: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![]);
    let set_b: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![]);
    
    assert!(set_a.is_disjoint(&set_b));
}

#[test]
fn test_is_disjoint_equal_length_empty_and_non_empty_with_common() {
    use indexmap::IndexSet;
    use std::collections::hash_map::RandomState;

    let set_a: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![]);
    let set_b: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![1]);
    
    assert!(set_a.is_disjoint(&set_b));
}

#[test]
fn test_is_disjoint_equal_length_reversed_sets() {
    use indexmap::IndexSet;
    use std::collections::hash_map::RandomState;

    let set_a: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![1, 2, 3]);
    let set_b: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![1, 2, 3]);
    
    assert!(!set_a.is_disjoint(&set_b));
}

