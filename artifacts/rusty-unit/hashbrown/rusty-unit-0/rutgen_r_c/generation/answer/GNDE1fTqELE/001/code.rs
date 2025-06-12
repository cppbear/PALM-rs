// Answer 0

#[test]
fn test_and_replace_entry_with_vacant_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    // Create a new HashMap
    let mut map: HashMap<&str, u32> = HashMap::new();

    // Try to access a vacant entry
    let entry = map.entry("poneyland").and_replace_entry_with(|_k, _v| panic!());

    // Verify that the entry is vacant
    match entry {
        Entry::Vacant(e) => {
            assert_eq!(e.key(), &"poneyland");
        }
        Entry::Occupied(_) => panic!(),
    }
}

#[test]
fn test_and_replace_entry_with_occupied_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    // Create a new HashMap and insert an initial value
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 42);

    // Access the occupied entry and update it
    let entry = map.entry("poneyland").and_replace_entry_with(|k, v| {
        assert_eq!(k, &"poneyland");
        assert_eq!(v, 42);
        Some(v + 1)
    });

    // Verify that the entry is occupied and updated
    match entry {
        Entry::Occupied(e) => {
            assert_eq!(e.key(), &"poneyland");
            assert_eq!(e.get(), &43);
        }
        Entry::Vacant(_) => panic!(),
    }

    // Ensure that the HashMap is updated
    assert_eq!(map["poneyland"], 43);
}

#[test]
fn test_and_replace_entry_with_remove_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    // Create a new HashMap and insert a value
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 42);

    // Access the entry and remove it
    let entry = map.entry("poneyland").and_replace_entry_with(|_k, _v| None);

    // Verify that the entry is vacant
    match entry {
        Entry::Vacant(e) => assert_eq!(e.key(), &"poneyland"),
        Entry::Occupied(_) => panic!(),
    }

    // Ensure that the key no longer exists
    assert!(!map.contains_key("poneyland"));
}

