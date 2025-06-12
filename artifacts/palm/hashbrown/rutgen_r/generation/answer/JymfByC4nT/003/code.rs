// Answer 0

#[test]
fn test_sub_assign_equal_length() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![3, 2, 1].into_iter().collect(); // Same length as a

    a -= &b;

    assert!(a.is_empty());
}

#[test]
fn test_sub_assign_same_elements_order_diff() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![4, 5, 6].into_iter().collect();
    let b: HashSet<_> = vec![6, 5, 4].into_iter().collect(); // Same elements in different order

    a -= &b;

    assert!(a.is_empty());
}

#[test]
fn test_sub_assign_elements_not_in_a() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![7, 8, 9].into_iter().collect();
    let b: HashSet<_> = vec![6, 5, 4].into_iter().collect(); // No elements in a

    a -= &b;

    let expected: HashSet<_> = vec![7, 8, 9].into_iter().collect();
    assert_eq!(a, expected);
}

#[test]
fn test_sub_assign_empty_a() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = HashSet::new(); // Empty set
    let b: HashSet<_> = vec![1, 2, 3].into_iter().collect(); // Non-empty set

    a -= &b;

    assert!(a.is_empty());
}

#[test]
fn test_sub_assign_a_contains_b() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 2, 3, 4].into_iter().collect();
    let b: HashSet<_> = vec![3, 4].into_iter().collect(); // b is a subset of a

    a -= &b;

    let expected: HashSet<_> = vec![1, 2].into_iter().collect();
    assert_eq!(a, expected);
}

