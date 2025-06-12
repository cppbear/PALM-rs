// Answer 0

#[test]
fn test_occupied_entry_get_existing_value() {
    use serde_json::json;
    use std::collections::BTreeMap;

    // Setting up the context for the test
    let mut map: BTreeMap<String, Value> = BTreeMap::new();
    let key = "serde".to_owned();
    let value = json!(12);
    
    // Inserting a value into the map
    map.insert(key.clone(), value.clone());

    // Creating an `OccupiedEntry`
    let occupied_entry = map.get_mut(&key).map(|v| OccupiedEntry { occupied: BTreeMap::occupied_entry(v) }).unwrap();

    // Testing the `get()` method
    assert_eq!(occupied_entry.get(), &value);
}

#[test]
#[should_panic(expected = "panicked at 'attempted to access a vacant entry'")]
fn test_occupied_entry_get_on_vacant_entry() {
    use serde_json::json;
    use std::collections::BTreeMap;

    // Setting up the context for the test
    let map: BTreeMap<String, Value> = BTreeMap::new();

    // Creating an `OccupiedEntry` for a non-existent key
    let occupied_entry = map.get_mut("non_existent_key").map(|v| OccupiedEntry { occupied: BTreeMap::occupied_entry(v) });

    // NOTE: This will panic as there is no entry for "non_existent_key"
    assert_eq!(occupied_entry.get(), &Value::Null); // This should panic
}

