// Answer 0

#[test]
fn test_get_occupied_entry() {
    use hashbrown::hash_set::{Entry, HashSet};

    let mut set: HashSet<&str> = HashSet::new();
    set.insert("poneyland");

    match set.entry("poneyland") {
        Entry::Vacant(_) => panic!("Expected occupied entry, but found vacant"),
        Entry::Occupied(entry) => {
            assert_eq!(entry.get(), &"poneyland");
        }
    }
}

#[test]
fn test_get_vacant_entry() {
    use hashbrown::hash_set::{Entry, HashSet};

    let mut set: HashSet<&str> = HashSet::new();

    match set.entry("notpresent") {
        Entry::Vacant(entry) => {
            entry.insert(); // This should not panic
        },
        Entry::Occupied(_) => {
            panic!("Expected vacant entry, but found occupied");
        }
    }

    // Verify that the value has been inserted
    assert!(set.contains("notpresent"));
}

