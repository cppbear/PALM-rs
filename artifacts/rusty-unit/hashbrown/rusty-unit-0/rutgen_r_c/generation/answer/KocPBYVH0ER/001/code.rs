// Answer 0

#[test]
fn test_or_insert_existing_entry() {
    use hashbrown::HashSet;

    let mut set: HashSet<&str> = HashSet::new();
    set.insert("poneyland"); // Insert an existing key

    // Ensure the entry is occupied
    match set.entry("poneyland") {
        hashbrown::hash_set::Entry::Occupied(_) => {
            // Call or_insert, should not panic and nothing should change 
            set.entry("poneyland").or_insert();
            assert!(set.contains("poneyland"));
            assert_eq!(set.len(), 1); // Length should still be 1
        }
        _ => panic!("Expected entry to be occupied"),
    }
}

#[test]
fn test_or_insert_empty_set() {
    use hashbrown::HashSet;

    let mut set: HashSet<&str> = HashSet::new();

    // When inserting in an empty set, it should be vacant
    match set.entry("poneyland") {
        hashbrown::hash_set::Entry::Vacant(entry) => {
            entry.insert(); // Should insert without panic
            assert!(set.contains("poneyland"));
            assert_eq!(set.len(), 1); // Length should be 1
        }
        _ => panic!("Expected entry to be vacant"),
    }
}

#[test]
fn test_or_insert_multiple_entries() {
    use hashbrown::HashSet;

    let mut set: HashSet<&str> = HashSet::new();
    // Insert the first entry
    set.entry("poneyland").or_insert();

    // Now try to insert a different entry
    match set.entry("paris") {
        hashbrown::hash_set::Entry::Vacant(entry) => {
            entry.insert(); // Should insert without panic
        }
        _ => panic!("Expected entry to be vacant"),
    }

    assert!(set.contains("poneyland"));
    assert!(set.contains("paris"));
    assert_eq!(set.len(), 2); // Length should be 2 now
}

