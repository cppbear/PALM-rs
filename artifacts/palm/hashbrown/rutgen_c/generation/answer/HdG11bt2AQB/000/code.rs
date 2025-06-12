// Answer 0

#[test]
fn test_capacity_with_initial_capacity() {
    // Initialize a HashSet with a specific capacity
    let set: HashSet<i32> = HashSet::with_capacity(100);
    // Verify that the capacity is at least the initialized value
    assert!(set.capacity() >= 100);
}

#[test]
fn test_capacity_on_empty_set() {
    // Initialize an empty HashSet
    let set: HashSet<i32> = HashSet::new();
    // Verify that the capacity is greater than or equal to zero
    assert!(set.capacity() >= 0);
}

