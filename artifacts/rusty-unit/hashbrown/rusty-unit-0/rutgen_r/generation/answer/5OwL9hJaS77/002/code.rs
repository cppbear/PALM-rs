// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    use hashbrown::HashMap;
    use std::collections::hash_map::RawEntryMut;

    // Create a HashMap and insert an initial value
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 3);

    // Create a RawEntryMut from the map
    let raw_entry: RawEntryMut<&str, u32> = map.raw_entry_mut();

    // Ensure the RawEntryMut is occupied and call or_insert
    let (key, value) = raw_entry.from_key("poneyland").or_insert("poneyland", 10);
    assert_eq!(key, &"poneyland");
    assert_eq!(*value, 3);

    // Change the value via the mutable reference and ensure it updates
    *value *= 2;
    assert_eq!(map["poneyland"], 6);
}

#[test]
#[should_panic]
fn test_or_insert_with_non_existent_key_should_panic() {
    use hashbrown::HashMap;
    use std::collections::hash_map::RawEntryMut;

    // Create a HashMap without the key
    let mut map: HashMap<&str, u32> = HashMap::new();
    
    // Create a RawEntryMut from the empty map
    let raw_entry: RawEntryMut<&str, u32> = map.raw_entry_mut();
    
    // This should panic since the entry is not occupied
    let _ = raw_entry.from_key("non_existent_key").or_insert("non_existent_key", 10);
}

