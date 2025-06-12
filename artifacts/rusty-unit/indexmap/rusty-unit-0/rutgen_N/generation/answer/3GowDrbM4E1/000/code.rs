// Answer 0

#[test]
fn test_is_subset_with_equal_sets() {
    use std::collections::hash_map::RandomState;
    use indexmap::indexset::IndexSet;

    let set_a: IndexSet<u32, RandomState> = IndexSet::from_iter(vec![1, 2, 3]);
    let set_b: IndexSet<u32, RandomState> = IndexSet::from_iter(vec![1, 2, 3]);

    assert!(set_a.is_subset(&set_b));
}

#[test]
fn test_is_subset_with_subset() {
    use std::collections::hash_map::RandomState;
    use indexmap::indexset::IndexSet;

    let set_a: IndexSet<u32, RandomState> = IndexSet::from_iter(vec![1, 2]);
    let set_b: IndexSet<u32, RandomState> = IndexSet::from_iter(vec![1, 2, 3]);

    assert!(set_a.is_subset(&set_b));
}

#[test]
fn test_is_subset_with_non_subset() {
    use std::collections::hash_map::RandomState;
    use indexmap::indexset::IndexSet;

    let set_a: IndexSet<u32, RandomState> = IndexSet::from_iter(vec![1, 4]);
    let set_b: IndexSet<u32, RandomState> = IndexSet::from_iter(vec![1, 2, 3]);

    assert!(!set_a.is_subset(&set_b));
}

#[test]
fn test_is_subset_with_empty_set() {
    use std::collections::hash_map::RandomState;
    use indexmap::indexset::IndexSet;

    let set_a: IndexSet<u32, RandomState> = IndexSet::new();
    let set_b: IndexSet<u32, RandomState> = IndexSet::from_iter(vec![1, 2, 3]);

    assert!(set_a.is_subset(&set_b));
}

#[test]
fn test_is_subset_with_large_set() {
    use std::collections::hash_map::RandomState;
    use indexmap::indexset::IndexSet;

    let set_a: IndexSet<u32, RandomState> = IndexSet::from_iter(0..100);
    let set_b: IndexSet<u32, RandomState> = IndexSet::from_iter(0..200);

    assert!(set_a.is_subset(&set_b));
}

