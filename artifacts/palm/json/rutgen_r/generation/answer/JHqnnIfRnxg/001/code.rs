// Answer 0

#[test]
fn test_insert_vacant_entry() {
    use serde_json::{json, Map, Value};
    use serde_json::map::Entry;

    let mut map = Map::new();

    // Case 1: Insert a basic string value
    match map.entry("key1") {
        Entry::Vacant(vacant) => {
            let insert_value = json!("value1");
            let result = vacant.insert(insert_value);
            assert_eq!(result, &mut json!("value1"));
        }
        Entry::Occupied(_) => panic!("Expected Vacant entry"),
    }

    // Case 2: Insert an integer value
    match map.entry("key2") {
        Entry::Vacant(vacant) => {
            let insert_value = json!(42);
            let result = vacant.insert(insert_value);
            assert_eq!(result, &mut json!(42));
        }
        Entry::Occupied(_) => panic!("Expected Vacant entry"),
    }

    // Case 3: Insert a boolean value
    match map.entry("key3") {
        Entry::Vacant(vacant) => {
            let insert_value = json!(true);
            let result = vacant.insert(insert_value);
            assert_eq!(result, &mut json!(true));
        }
        Entry::Occupied(_) => panic!("Expected Vacant entry"),
    }

    // Case 4: Insert a nested JSON value
    match map.entry("key4") {
        Entry::Vacant(vacant) => {
            let insert_value = json!({"nested_key": "nested_value"});
            let result = vacant.insert(insert_value);
            assert_eq!(result, &mut json!({"nested_key": "nested_value"}));
        }
        Entry::Occupied(_) => panic!("Expected Vacant entry"),
    }
}

#[test]
#[should_panic(expected = "Expected Vacant entry")]
fn test_insert_occupied_entry() {
    use serde_json::{json, Map, Value};
    use serde_json::map::Entry;

    let mut map = Map::new();
    // Initially populate the map
    map.insert("key1".to_string(), json!("existing_value"));

    // Trying to insert into an occupied entry
    match map.entry("key1") {
        Entry::Vacant(_) => panic!("Expected Occupied entry"),
        Entry::Occupied(mut occupied) => {
            occupied.insert(json!("new_value"));
        }
    }
}

