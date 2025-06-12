// Answer 0

#[test]
fn test_sub_assign_different_sets() {
    use hashbrown::HashSet;

    // Create a HashSet with 5 elements
    let mut set_a: HashSet<_> = vec![1, 2, 3, 4, 5].into_iter().collect();
    // Create a HashSet with 2 elements that is a subset of set_a
    let set_b: HashSet<_> = vec![3, 4].into_iter().collect();

    // Perform the subtraction operation
    set_a -= &set_b;

    // Check that set_a contains only the expected elements after subtraction
    let expected = [1, 2, 5];
    let mut count = 0;
    for x in &set_a {
        assert!(expected.contains(x));
        count += 1;
    }
    assert_eq!(count, expected.len());
}

#[test]
fn test_sub_assign_equal_size_sets() {
    use hashbrown::HashSet;

    // Create a HashSet with 3 elements
    let mut set_a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    // Create a HashSet with 3 elements that is equal to set_a
    let set_b: HashSet<_> = vec![1, 2, 3].into_iter().collect();

    // Perform the subtraction operation
    set_a -= &set_b;

    // Check that set_a is now empty
    assert!(set_a.is_empty());
}

#[test]
fn test_sub_assign_larger_rhs_set() {
    use hashbrown::HashSet;

    // Create a HashSet with 3 elements
    let mut set_a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    // Create a HashSet with 4 elements (larger than set_a)
    let set_b: HashSet<_> = vec![1, 2, 3, 4].into_iter().collect();

    // Perform the subtraction operation
    set_a -= &set_b;

    // Check that set_a remains the same since rhs is larger
    let expected = [1, 2, 3];
    let mut count = 0;
    for x in &set_a {
        assert!(expected.contains(x));
        count += 1;
    }
    assert_eq!(count, expected.len());
}

#[test]
fn test_sub_assign_empty_rhs() {
    use hashbrown::HashSet;

    // Create a HashSet with 3 elements
    let mut set_a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    // Create an empty HashSet
    let set_b: HashSet<_> = HashSet::new();

    // Perform the subtraction operation
    set_a -= &set_b;

    // Check that set_a remains unchanged
    let expected = [1, 2, 3];
    let mut count = 0;
    for x in &set_a {
        assert!(expected.contains(x));
        count += 1;
    }
    assert_eq!(count, expected.len());
}

