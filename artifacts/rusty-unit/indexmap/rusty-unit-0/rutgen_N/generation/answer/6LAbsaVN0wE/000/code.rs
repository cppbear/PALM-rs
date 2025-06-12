// Answer 0

#[test]
fn test_symmetric_difference_no_common_elements() {
    use indexmap::IndexSet;
    use std::collections::hash_map::RandomState;

    let set_a: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![1, 2, 3]);
    let set_b: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![4, 5, 6]);

    let diff: Vec<_> = set_a.symmetric_difference(&set_b).collect();
    assert_eq!(diff, vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_symmetric_difference_with_common_elements() {
    use indexmap::IndexSet;
    use std::collections::hash_map::RandomState;

    let set_a: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![1, 2, 3]);
    let set_b: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![2, 3, 4]);

    let diff: Vec<_> = set_a.symmetric_difference(&set_b).collect();
    assert_eq!(diff, vec![1, 4]);
}

#[test]
fn test_symmetric_difference_empty_set_a() {
    use indexmap::IndexSet;
    use std::collections::hash_map::RandomState;

    let set_a: IndexSet<i32, RandomState> = IndexSet::new();
    let set_b: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![4, 5, 6]);

    let diff: Vec<_> = set_a.symmetric_difference(&set_b).collect();
    assert_eq!(diff, vec![4, 5, 6]);
}

#[test]
fn test_symmetric_difference_empty_set_b() {
    use indexmap::IndexSet;
    use std::collections::hash_map::RandomState;

    let set_a: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![1, 2, 3]);
    let set_b: IndexSet<i32, RandomState> = IndexSet::new();

    let diff: Vec<_> = set_a.symmetric_difference(&set_b).collect();
    assert_eq!(diff, vec![1, 2, 3]);
}

#[test]
fn test_symmetric_difference_both_empty() {
    use indexmap::IndexSet;
    use std::collections::hash_map::RandomState;

    let set_a: IndexSet<i32, RandomState> = IndexSet::new();
    let set_b: IndexSet<i32, RandomState> = IndexSet::new();

    let diff: Vec<_> = set_a.symmetric_difference(&set_b).collect();
    assert_eq!(diff, vec![]);
}

