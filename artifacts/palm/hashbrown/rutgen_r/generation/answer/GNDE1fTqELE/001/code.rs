// Answer 0

#[test]
fn test_and_replace_entry_with_vacant() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();

    // Create an Entry for a key that does not exist in the map.
    let entry = map
        .entry("missing")
        .and_replace_entry_with(|_k, _v| panic!("This should not be called"));

    match entry {
        Entry::Vacant(e) => {
            assert_eq!(e.key(), &"missing");
        }
        Entry::Occupied(_) => panic!("Expected Vacant entry"),
    }
}

#[test]
fn test_and_replace_entry_with_vacant_after_insert() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();

    // Insert a value to ensure the next entry is not vacant.
    map.insert("present", 42);

    // Now we try to call and_replace_entry_with on a new key that hasn't been inserted.
    let entry = map
        .entry("absent")
        .and_replace_entry_with(|_k, _v| panic!("This should not be called"));

    match entry {
        Entry::Vacant(e) => {
            assert_eq!(e.key(), &"absent");
        }
        Entry::Occupied(_) => panic!("Expected Vacant entry"),
    }
}

