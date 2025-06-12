// Answer 0

#[test]
fn test_remove_from_occupied_entry() {
    use hashbrown::HashSet;
    use hashbrown::hash_set::Entry;

    let mut set: HashSet<&str> = HashSet::new();
    assert!(set.is_empty() && set.capacity() == 0);
    
    // Insert an element
    set.entry("poneyland").or_insert();
    let capacity_before_remove = set.capacity();

    // Now remove the element and check the return value
    if let Entry::Occupied(o) = set.entry("poneyland") {
        let removed_value = o.remove();
        assert_eq!(removed_value, "poneyland");
    }

    // Ensure the set is now empty
    assert!(!set.contains("poneyland"));
    assert!(set.len() == 0 && set.capacity() == capacity_before_remove);
}

#[test]
#[should_panic]
fn test_remove_from_vacant_entry() {
    use hashbrown::HashSet;
    use hashbrown::hash_set::Entry;

    let mut set: HashSet<&str> = HashSet::new();
    
    // Attempting to remove from a vacant entry should panic
    if let Entry::Vacant(_) = set.entry("nonexistent") {
        panic!("Trying to remove from a vacant entry should panic");
    }
}

#[test]
fn test_remove_and_check_capacity() {
    use hashbrown::HashSet;
    use hashbrown::hash_set::Entry;

    let mut set: HashSet<&str> = HashSet::new();
    set.entry("poneyland").or_insert();
    let capacity_before_remove = set.capacity();

    if let Entry::Occupied(o) = set.entry("poneyland") {
        let removed_value = o.remove();
        assert_eq!(removed_value, "poneyland");
    }

    // Check capacity after removal
    assert!(set.capacity() == capacity_before_remove);
}

