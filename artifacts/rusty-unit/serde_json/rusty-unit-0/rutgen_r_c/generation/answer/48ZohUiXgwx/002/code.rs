// Answer 0

#[test]
fn test_or_insert_vacant() {
    use std::collections::BTreeMap; // or IndexMap depending on the feature flag
    use serde_json::Value;

    // Create a new map
    let mut map: BTreeMap<String, Value> = BTreeMap::new();

    // Create a VacantEntry
    let key = String::from("serde");
    let vacant_entry = VacantEntry { vacant: map.entry(key.clone()) };

    // Call or_insert on a Vacant entry
    let inserted_value = vacant_entry.insert(Value::Number(serde_json::Number::from(12)));

    // Verify that the entry was inserted and returned correctly
    assert_eq!(*inserted_value, Value::Number(serde_json::Number::from(12)));
    assert_eq!(map.get(&key), Some(&Value::Number(serde_json::Number::from(12))));
}

#[test]
fn test_or_insert_occupied() {
    use std::collections::BTreeMap; // or IndexMap depending on the feature flag
    use serde_json::Value;

    let mut map: BTreeMap<String, Value> = BTreeMap::new();

    // Pre-insert a value to create an occupied entry
    let key = String::from("serde");
    map.insert(key.clone(), Value::Number(serde_json::Number::from(10)));

    // Create an OccupiedEntry
    let occupied_entry = OccupiedEntry { occupied: map.entry(key.clone()).or_insert(Value::Null) };

    // Call or_insert on an Occupied entry with a different value
    let returned_value = occupied_entry.insert(Value::Number(serde_json::Number::from(12)));

    // Verify the return value and the state of the map
    assert_eq!(*returned_value, Value::Number(12));
    assert_eq!(map.get(&key), Some(&Value::Number(12)));
}

