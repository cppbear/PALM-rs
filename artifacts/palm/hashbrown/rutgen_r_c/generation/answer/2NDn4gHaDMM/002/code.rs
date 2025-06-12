// Answer 0

#[test]
fn test_insert_occupied_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    // Set up a HashMap and populate it with one key-value pair
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("key1", 10);

    // Retrieve the entry in an Occupied state
    let entry = map.entry("key1");

    // Assert that the entry is Occupied
    if let Entry::Occupied(mut occupied_entry) = entry {
        // Insert a new value into the occupied entry
        let returned_entry = occupied_entry.insert(20);

        // Assert that the key remains the same
        assert_eq!(returned_entry.key(), &"key1");
        // Assert that the value has been replaced with the new value
        assert_eq!(map.get("key1"), Some(&20));
    } else {
        panic!("Expected entry to be Occupied");
    }
}

#[test]
fn test_insert_new_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    // Set up a HashMap
    let mut map: HashMap<&str, u32> = HashMap::new();

    // Retrieve an entry for a key that does not exist (Vacant entry)
    let entry = map.entry("key2");

    // Assert that the entry is Vacant
    if let Entry::Vacant(vacant_entry) = entry {
        // Insert a new value into the vacant entry
        let returned_entry = vacant_entry.insert(30);

        // Assert that the key is correct
        assert_eq!(returned_entry.key(), &"key2");
        // Assert that the new key-value pair has been inserted
        assert_eq!(map.get("key2"), Some(&30));
    } else {
        panic!("Expected entry to be Vacant");
    }
}

