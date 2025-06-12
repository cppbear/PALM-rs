// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    use hashbrown::HashMap;
    use std::collections::hash_map::RandomState;

    let mut map: HashMap<&str, u32> = HashMap::new();

    // Insert an initial value
    map.insert("poneyland", 3);

    // Create a RawEntryMut from the key
    let entry = map.raw_entry_mut().from_key("poneyland");

    // Use or_insert to ensure the value is correct
    let (key_ref, val_ref) = entry.or_insert("poneyland", 10);
    assert_eq!(*val_ref, 3);
    assert_eq!(*key_ref, "poneyland");
}

#[test]
fn test_or_insert_with_vacant_entry() {
    use hashbrown::HashMap;
    use std::collections::hash_map::RandomState;

    let mut map: HashMap<&str, u32> = HashMap::new();

    // Create a RawEntryMut from a key that is not inserted
    let entry = map.raw_entry_mut().from_key("poneyland");

    // Insert value using or_insert
    let (key_ref, val_ref) = entry.or_insert("poneyland", 3);
    assert_eq!(*val_ref, 3);
    assert_eq!(*key_ref, "poneyland");

    // Ensure the map reflects the inserted value
    assert_eq!(map["poneyland"], 3);
}

#[test]
fn test_or_insert_modify_existing_value() {
    use hashbrown::HashMap;
    use std::collections::hash_map::RandomState;

    let mut map: HashMap<&str, u32> = HashMap::new();

    // Insert an initial value
    map.insert("poneyland", 3);

    // Create a RawEntryMut and modify the existing value
    let (key_ref, val_ref) = map.raw_entry_mut().from_key("poneyland").or_insert("poneyland", 10);
    *val_ref *= 2;

    // Validate the value in the map was updated correctly
    assert_eq!(map["poneyland"], 6);
}

