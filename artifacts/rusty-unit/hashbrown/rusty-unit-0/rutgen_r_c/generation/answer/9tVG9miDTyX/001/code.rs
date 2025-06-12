// Answer 0

#[test]
fn test_shrink_to_valid_case() {
    use hashbrown::HashSet;

    let mut set = HashSet::with_capacity(100);
    set.insert(1);
    set.insert(2);
    // The capacity should be at least 100 as per the initializer.
    assert!(set.allocation_size() >= 100);
    set.shrink_to(10);
    // After shrinking, the capacity should be at least 10.
    assert!(set.allocation_size() >= 10);
    set.shrink_to(0);
    // Should still be at least as large as current elements, which is 2.
    assert!(set.allocation_size() >= 2);
}

#[should_panic]
#[test]
fn test_shrink_to_panics_on_smaller_capacity() {
    use hashbrown::HashSet;

    let mut set = HashSet::with_capacity(10);
    set.insert(1);
    set.insert(2);
    // Ensure initial capacity is adequate before testing panic condition
    assert!(set.allocation_size() >= 10);
    // Attempt to shrink to a capacity of 20, which is larger than current capacity
    set.shrink_to(20);
}

#[test]
fn test_shrink_to_no_op_on_valid_capacity() {
    use hashbrown::HashSet;

    let mut set = HashSet::with_capacity(20);
    set.insert(1);
    set.insert(2);
    // Initial capacity should be at least 20
    assert!(set.allocation_size() >= 20);
    let original_capacity = set.allocation_size();
    // Shrink to 15, it should not change as it's already larger
    set.shrink_to(15);
    assert_eq!(original_capacity, set.allocation_size());
}

