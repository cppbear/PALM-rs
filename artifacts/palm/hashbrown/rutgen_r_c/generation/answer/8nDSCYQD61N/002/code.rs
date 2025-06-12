// Answer 0

#[test]
fn test_and_modify_on_occupied_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::EntryRef;

    // Initialize a HashMap with one entry
    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("key".to_owned(), 1);

    // Create an EntryRef pointing to the occupied entry
    let entry_ref: EntryRef<_, _, _, _> = map.entry_ref("key").unwrap();

    // Use the and_modify method to modify the value
    let modified_entry_ref = entry_ref.and_modify(|value| {
        *value += 1; // Increment the value
    });

    // Validate that the entry is still occupied and the value was modified
    if let EntryRef::Occupied(occupied_entry) = modified_entry_ref {
        assert_eq!(occupied_entry.get(), &2); // Check if the new value is 2
    } else {
        panic!("Expected EntryRef::Occupied but got Vacant");
    }
}

#[test]
fn test_and_modify_on_vacant_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::EntryRef;

    // Initialize a HashMap without any entries
    let mut map: HashMap<String, u32> = HashMap::new();

    // Create an EntryRef pointing to a vacant entry
    let entry_ref: EntryRef<_, _, _, _> = map.entry_ref("nonexistent_key").unwrap();

    // Use the and_modify method on a vacant entry
    let modified_entry_ref = entry_ref.and_modify(|_value| {
        panic!("This should not be called for a vacant entry");
    });

    // Validate that the entry is still vacant
    if let EntryRef::Vacant(_) = modified_entry_ref {
        // Passes as expected
    } else {
        panic!("Expected EntryRef::Vacant but got Occupied");
    }
}

