// Answer 0

#[test]
fn test_or_insert_with_vacant_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, u32> = HashMap::new();

    // Test inserting a value for a vacant entry
    let value_ref = map.entry_ref("vacant_key").or_insert_with(|| 42);
    assert_eq!(*value_ref, 42);
    assert_eq!(map["vacant_key"], 42);
}

#[test]
fn test_or_insert_with_existing_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, u32> = HashMap::new();

    // Insert a value into the map
    map.insert("existing_key".to_string(), 10);

    // Ensure the existing entry is modified
    let value_ref = map.entry_ref("existing_key").or_insert_with(|| 20);
    *value_ref *= 2; // Modify the existing value
    assert_eq!(*value_ref, 20);
    assert_eq!(map["existing_key"], 20);
}

