// Answer 0

#[test]
fn test_hash_set_new_empty() {
    use hashbrown::HashSet;

    // Create an empty HashSet
    let set: HashSet<i32> = HashSet::new();

    // Verify that the set is empty
    assert!(set.is_empty());
}

#[test]
fn test_hash_set_new_capacity() {
    use hashbrown::HashSet;

    // Create an empty HashSet with default capacity
    let set: HashSet<i32> = HashSet::new();

    // Ensure its capacity is zero and doesn't allocate
    assert_eq!(set.len(), 0);
    assert!(set.capacity() == 0);
}

