// Answer 0

#[test]
fn test_remove_entry() {
    use hashbrown::HashSet;
    use hashbrown::hash_set::Entry;

    // Initialize the HashSet
    let mut set: HashSet<&str> = HashSet::new();
    assert!(set.is_empty() && set.capacity() == 0);

    // Insert an element
    set.entry("poneyland").or_insert();
    let capacity_before_remove = set.capacity();

    // Remove the occupied entry
    if let Entry::Occupied(o) = set.entry("poneyland") {
        assert_eq!(o.remove(), "poneyland");
    }

    // Check that the element was removed
    assert_eq!(set.contains("poneyland"), false);
    // Verify that the capacity remains unchanged
    assert!(set.len() == 0 && set.capacity() == capacity_before_remove);
}

