// Answer 0

#[test]
fn test_remove_entry_from_empty_map() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    assert!(map.is_empty());

    // Attempt to remove an entry from an empty map; it should not panic.
    let remove_result = {
        if let Entry::Vacant(v) = map.entry("nonexistent_key") {
            assert!(v.insert(0).is_none());
            v.remove_entry()
        } else {
            panic!("Expected the entry to be vacant");
        }
    };

    // The assert may not trigger if the entry is never occupied
    assert!(remove_result.is_none());
}

#[test]
fn test_remove_entry_from_non_empty_map() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 12);
    assert_eq!(map.len(), 1);

    if let Entry::Occupied(o) = map.entry("poneyland") {
        assert_eq!(o.remove_entry(), ("poneyland", 12)); // Valid removal
    } else {
        panic!("Expected the entry to be occupied");
    }

    assert!(!map.contains_key("poneyland"));
    assert!(map.is_empty());
}

#[test]
fn test_remove_entry_twice() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("unique_key", 42);
    
    // First removal
    if let Entry::Occupied(o) = map.entry("unique_key") {
        assert_eq!(o.remove_entry(), ("unique_key", 42));
    }

    // Check the key is no longer present
    assert!(!map.contains_key("unique_key"));
    
    // Second removal attempt, should succeed again with appropriate logic
    if let Entry::Vacant(v) = map.entry("unique_key") {
        assert!(v.insert(1).is_none());
    }
}

#[test]
#[should_panic]
fn test_remove_entry_from_occupied_key() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    // This test is meant to panic.
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("panic_key", 100);
    
    // This will panic because the remove method needs an occupied entry
    if let Entry::Occupied(o) = map.entry("panic_key") {
        assert_eq!(o.remove_entry(), ("panic_key", 100));
    }

    // Another attempt to remove, after the key has been removed
    if let Entry::Occupied(o) = map.entry("panic_key") {
        o.remove_entry(); // This will panic
    }
}

