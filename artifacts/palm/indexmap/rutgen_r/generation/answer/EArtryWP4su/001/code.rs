// Answer 0

#[test]
fn test_set_difference_non_empty_sets() {
    use indexmap::IndexSet;

    let set_a: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3, 4]);
    let set_b: IndexSet<i32> = IndexSet::from_iter(vec![3, 4, 5, 6]);

    let result = set_a.sub(&set_b);
    let expected: IndexSet<i32> = IndexSet::from_iter(vec![1, 2]);

    assert_eq!(result, expected);
}

#[test]
fn test_set_difference_empty_other_set() {
    use indexmap::IndexSet;

    let set_a: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3, 4]);
    let set_b: IndexSet<i32> = IndexSet::new();

    let result = set_a.sub(&set_b);
    let expected: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3, 4]);

    assert_eq!(result, expected);
}

#[test]
fn test_set_difference_empty_self_set() {
    use indexmap::IndexSet;

    let set_a: IndexSet<i32> = IndexSet::new();
    let set_b: IndexSet<i32> = IndexSet::from_iter(vec![3, 4]);

    let result = set_a.sub(&set_b);
    let expected: IndexSet<i32> = IndexSet::new();

    assert_eq!(result, expected);
}

#[test]
fn test_set_difference_identical_sets() {
    use indexmap::IndexSet;

    let set_a: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3]);
    let set_b: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3]);

    let result = set_a.sub(&set_b);
    let expected: IndexSet<i32> = IndexSet::new();

    assert_eq!(result, expected);
}

#[test]
fn test_set_difference_no_common_elements() {
    use indexmap::IndexSet;

    let set_a: IndexSet<i32> = IndexSet::from_iter(vec![1, 2]);
    let set_b: IndexSet<i32> = IndexSet::from_iter(vec![3, 4]);

    let result = set_a.sub(&set_b);
    let expected: IndexSet<i32> = IndexSet::from_iter(vec![1, 2]);

    assert_eq!(result, expected);
}

