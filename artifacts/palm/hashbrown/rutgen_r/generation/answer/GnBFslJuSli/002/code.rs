// Answer 0

#[test]
fn test_bitor_assign_union_with_non_overlapping_sets() {
    use hashbrown::HashSet;

    // Creating two non-overlapping sets
    let mut a: HashSet<_> = vec![1, 2].into_iter().collect();
    let b: HashSet<_> = vec![3, 4].into_iter().collect();

    a |= &b;

    let expected = [1, 2, 3, 4];
    let mut i = 0;
    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_bitor_assign_union_with_partial_overlap() {
    use hashbrown::HashSet;

    // Creating sets with partial overlap
    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![3, 4, 5].into_iter().collect();

    a |= &b;

    let expected = [1, 2, 3, 4, 5];
    let mut i = 0;
    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_bitor_assign_union_with_empty_set() {
    use hashbrown::HashSet;

    // Testing with an empty rhs set
    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = HashSet::new();

    a |= &b;

    let expected = [1, 2, 3];
    let mut i = 0;
    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_bitor_assign_union_with_disjoint_sets() {
    use hashbrown::HashSet;

    // Testing with completely disjoint sets
    let mut a: HashSet<_> = vec![10, 20, 30].into_iter().collect();
    let b: HashSet<_> = vec![40, 50, 60].into_iter().collect();

    a |= &b;

    let expected = [10, 20, 30, 40, 50, 60];
    let mut i = 0;
    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_bitor_assign_self_contains_all_rhs() {
    use hashbrown::HashSet;

    // Creating a set where 'a' contains all items in 'b'
    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![1, 2].into_iter().collect();

    a |= &b;

    let expected = [1, 2, 3];
    let mut i = 0;
    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

