// Answer 0

#[test]
fn test_sub_assign_different_lengths() {
    use hashbrown::HashSet;

    // Set up the first set with more elements than the second set
    let mut a: HashSet<_> = vec![1, 2, 3, 4, 5].into_iter().collect();
    let b: HashSet<_> = vec![3, 4].into_iter().collect();

    a -= &b;

    let expected = [1, 2, 5];
    let mut i = 0;

    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_sub_assign_equal_lengths() {
    use hashbrown::HashSet;

    // Set up both sets to have equal lengths
    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![3, 4, 5].into_iter().collect();

    a -= &b;

    let expected = [1, 2];
    let mut i = 0;

    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_sub_assign_no_common_elements() {
    use hashbrown::HashSet;

    // Set up two sets with no common elements
    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![4, 5].into_iter().collect();

    a -= &b;

    let expected = [1, 2, 3];
    let mut i = 0;

    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_sub_assign_rhs_is_empty() {
    use hashbrown::HashSet;

    // Set the right-hand set to be empty
    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = HashSet::new(); // Empty set

    a -= &b;

    let expected = [1, 2, 3];
    let mut i = 0;

    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

