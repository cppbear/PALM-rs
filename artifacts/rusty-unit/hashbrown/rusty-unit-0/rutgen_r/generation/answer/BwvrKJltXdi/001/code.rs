// Answer 0

#[test]
fn test_replace_entry_with_occupied_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    // Initialize a HashMap and insert an entry
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 42);

    // Fetch the occupied entry and replace its value using replace_entry_with
    let entry = match map.entry("poneyland") {
        Entry::Occupied(e) => {
            e.replace_entry_with(|k, v| {
                assert_eq!(k, &"poneyland");
                assert_eq!(v, 42);
                Some(v + 1) // Increment the value
            })
        }
        Entry::Vacant(_) => panic!(),
    };

    // Validate that the entry is still occupied and the value is updated
    match entry {
        Entry::Occupied(e) => {
            assert_eq!(e.key(), &"poneyland");
            assert_eq!(e.get(), &43);
        }
        Entry::Vacant(_) => panic!(),
    }

    // Ensure the map reflects the updated value
    assert_eq!(map["poneyland"], 43);
}

#[test]
fn test_replace_entry_with_remove_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    // Initialize a HashMap and insert an entry
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 42);

    // Fetch the occupied entry and remove it using replace_entry_with
    let entry = match map.entry("poneyland") {
        Entry::Occupied(e) => e.replace_entry_with(|_k, _v| None), // Remove the entry
        Entry::Vacant(_) => panic!(),
    };

    // Validate that the entry is now vacant
    match entry {
        Entry::Vacant(e) => {
            assert_eq!(e.key(), &"poneyland");
        }
        Entry::Occupied(_) => panic!(),
    }

    // Ensure the map no longer contains the key
    assert!(!map.contains_key("poneyland"));
}

