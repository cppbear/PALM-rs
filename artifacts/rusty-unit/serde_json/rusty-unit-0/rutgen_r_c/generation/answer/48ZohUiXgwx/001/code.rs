// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    use std::collections::BTreeMap;
    
    let mut map: BTreeMap<String, Value> = BTreeMap::new();
    map.insert("key".to_string(), Value::Number(serde_json::Number::from(42)));

    // Create an occupied entry
    let occupied_entry = Entry::Occupied(OccupiedEntry {
        occupied: map.get_mut("key").map(|v| {
            OccupiedEntryImpl {
                occupied: v,
            }
        }).unwrap(),
    });

    // Call or_insert on an occupied entry, should return a mutable reference to the existing entry
    let result: &mut Value = occupied_entry.or_insert(Value::Number(serde_json::Number::from(100)));

    // Verify that the value remains the same and is still mutable
    assert_eq!(result, &mut Value::Number(serde_json::Number::from(42)));
    *result = Value::Number(serde_json::Number::from(100)); // Modify the value

    // Verify that the map reflects the updated value
    assert_eq!(map.get("key"), Some(&Value::Number(serde_json::Number::from(100))));
}

#[test]
fn test_or_insert_with_vacant_entry() {
    use std::collections::BTreeMap;

    let mut map: BTreeMap<String, Value> = BTreeMap::new();

    // Create a vacant entry
    let vacant_entry = Entry::Vacant(VacantEntry {
        vacant: map.entry("new_key".to_string()),
    });

    // Call or_insert on a vacant entry, should insert the new value
    let result: &mut Value = vacant_entry.or_insert(Value::String("default".to_string()));

    // Verify that the value has been inserted
    assert_eq!(result, &mut Value::String("default".to_string()));
    assert_eq!(map.get("new_key"), Some(&Value::String("default".to_string())));
}

