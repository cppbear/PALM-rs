// Answer 0

#[test]
fn test_symmetric_difference_with_non_existent_items() {
    use hashbrown::HashSet;

    // Initialize sets a and b
    let mut a: HashSet<_> = vec![1, 2].into_iter().collect();
    let b: HashSet<_> = vec![3, 4, 5].into_iter().collect();

    // Perform symmetric difference operation
    a ^= &b;

    // Check the contents of a after symmetric difference
    let expected: Vec<_> = vec![1, 2, 3, 4, 5];
    let mut i = 0;
    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_symmetric_difference_with_common_elements() {
    use hashbrown::HashSet;

    // Initialize sets a and b with common elements
    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![3, 4, 5].into_iter().collect();

    // Perform symmetric difference operation
    a ^= &b;

    // Check the contents of a after symmetric difference
    let expected: Vec<_> = vec![1, 2, 4, 5];
    let mut i = 0;
    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_symmetric_difference_empty_rhs() {
    use hashbrown::HashSet;

    // Initialize set a and an empty set b
    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = HashSet::new();

    // Perform symmetric difference operation
    a ^= &b;

    // Check that a remains unchanged
    let expected: Vec<_> = vec![1, 2, 3];
    let mut i = 0;
    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_symmetric_difference_with_empty_self() {
    use hashbrown::HashSet;

    // Initialize an empty set a and set b
    let mut a: HashSet<_> = HashSet::new();
    let b: HashSet<_> = vec![1, 2].into_iter().collect();

    // Perform symmetric difference operation
    a ^= &b;

    // Check the contents of a after symmetric difference
    let expected: Vec<_> = vec![1, 2];
    let mut i = 0;
    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

