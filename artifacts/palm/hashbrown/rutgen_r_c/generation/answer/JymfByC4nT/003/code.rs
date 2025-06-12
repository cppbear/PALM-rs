// Answer 0

#[test]
fn test_sub_assign_equal_length_sets() {
    use hashbrown::HashSet;

    // Initialize two HashSet instances with the same elements
    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![1, 2, 3].into_iter().collect();

    // Perform the subtraction operation
    a -= &b;

    // After subtraction, `a` should be empty
    assert!(a.is_empty());
}

#[test]
fn test_sub_assign_with_non_matching_element() {
    use hashbrown::HashSet;

    // Initialize two HashSet instances
    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![3, 4, 5].into_iter().collect();

    // Perform the subtraction operation
    a -= &b;

    // After subtraction, `a` should contain only 1 and 2
    let expected = [1, 2];
    let mut i = 0;
    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_sub_assign_large_equal_size_sets() {
    use hashbrown::HashSet;

    // Initialize two HashSet instances with 1000 identical elements
    let mut a: HashSet<_> = (1..=1000).collect();
    let b: HashSet<_> = (1..=1000).collect();

    // Perform the subtraction operation
    a -= &b;

    // After subtraction, `a` should be empty
    assert!(a.is_empty());
}

