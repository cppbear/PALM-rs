// Answer 0

#[test]
fn test_symmetric_difference_with_disjoint_sets() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexSet;

    let set1: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![1, 2, 3]);
    let set2: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![4, 5, 6]);

    let result: Vec<_> = set1.symmetric_difference(&set2).collect();
    assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_symmetric_difference_with_overlap() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexSet;

    let set1: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![1, 2, 3]);
    let set2: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![3, 4, 5]);

    let result: Vec<_> = set1.symmetric_difference(&set2).collect();
    assert_eq!(result, vec![1, 2, 4, 5]);
}

#[test]
fn test_symmetric_difference_with_identical_sets() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexSet;

    let set1: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![1, 2, 3]);
    let set2: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![1, 2, 3]);

    let result: Vec<_> = set1.symmetric_difference(&set2).collect();
    assert_eq!(result, Vec::<i32>::new());
}

#[test]
fn test_symmetric_difference_with_empty_set() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexSet;

    let set1: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![1, 2, 3]);
    let set2: IndexSet<i32, RandomState> = IndexSet::new();

    let result: Vec<_> = set1.symmetric_difference(&set2).collect();
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn test_symmetric_difference_with_all_elements_empty() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexSet;

    let set1: IndexSet<i32, RandomState> = IndexSet::new();
    let set2: IndexSet<i32, RandomState> = IndexSet::new();

    let result: Vec<_> = set1.symmetric_difference(&set2).collect();
    assert_eq!(result, Vec::<i32>::new());
}

