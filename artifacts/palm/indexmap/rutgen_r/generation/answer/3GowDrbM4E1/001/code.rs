// Answer 0

#[test]
fn test_is_subset_equal_length() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexSet;

    // Create `self` and `other` with the same elements and lengths
    let set_one: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![1, 2, 3]);
    let set_two: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![1, 2, 3]);

    // Test that set_one is a subset of set_two
    assert!(set_one.is_subset(&set_two));
}

#[test]
fn test_is_subset_empty_sets() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexSet;

    // Create two empty IndexSets
    let set_one: IndexSet<i32, RandomState> = IndexSet::new();
    let set_two: IndexSet<i32, RandomState> = IndexSet::new();

    // Test that the empty set is a subset of another empty set
    assert!(set_one.is_subset(&set_two));
}

#[test]
fn test_is_subset_not_a_subset() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexSet;

    // Create a smaller set and a larger set that do not maintain the subset property
    let set_one: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![1, 2]);
    let set_two: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![1, 2, 3]);

    // Check that set_one is indeed a subset of set_two
    assert!(set_one.is_subset(&set_two));
}

