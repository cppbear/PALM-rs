// Answer 0

#[test]
fn test_insert_vacant_entry() {
    use alloc::collections::btree_map::BTreeMap;
    use crate::value::Value;

    let mut map: BTreeMap<String, Value> = BTreeMap::new();
    let key = String::from("test_key");
    
    // Create a VacantEntry manually
    let vacant_entry = VacantEntry {
        vacant: map.entry(key.clone()).or_insert_with(|| Value::Null).empty(),
    };

    // Insert a value and check the result
    let inserted_value = Value::String("test_value".to_string());
    let result = vacant_entry.insert(inserted_value);

    // Verify that the inserted value can be retrieved from the map
    assert_eq!(result, &mut Value::String("test_value".to_string()));
    assert_eq!(map.get(&key), Some(&Value::String("test_value".to_string())));
}

#[test]
#[should_panic(expected = "attempt to insert when entry is occupied")]
fn test_insert_vacant_entry_with_occupied() {
    use alloc::collections::btree_map::BTreeMap;
    use crate::value::Value;

    let mut map: BTreeMap<String, Value> = BTreeMap::new();
    let key = String::from("test_key");
    
    // Create an occupied entry first
    map.insert(key.clone(), Value::String("already_exists".to_string()));

    // Attempt to create a VacantEntry for the existing key
    let vacant_entry = VacantEntry {
        vacant: map.entry(key.clone()).or_insert_with(|| Value::Null).empty(),
    };

    // Attempting to insert a value should panic
    vacant_entry.insert(Value::String("new_value".to_string()));
}

