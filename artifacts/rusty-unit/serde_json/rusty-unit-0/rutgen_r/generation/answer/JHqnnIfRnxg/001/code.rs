// Answer 0

#[test]
fn test_insert_vacant_entry() {
    use serde_json::{Map, Value, json};
    use serde_json::map::Entry;

    let mut map = Map::new();

    // Test inserting a value into a vacant entry
    match map.entry("key1") {
        Entry::Vacant(vacant) => {
            let value_ref: &mut Value = vacant.insert(json!("value1"));
            assert_eq!(*value_ref, json!("value1"));
            assert_eq!(map.get("key1"), Some(&json!("value1")));
        }
        Entry::Occupied(_) => panic!("Expected vacant entry, but found occupied entry"),
    }

    // Test inserting another value into a new vacant entry
    match map.entry("key2") {
        Entry::Vacant(vacant) => {
            let value_ref: &mut Value = vacant.insert(json!(42));
            assert_eq!(*value_ref, json!(42));
            assert_eq!(map.get("key2"), Some(&json!(42)));
        }
        Entry::Occupied(_) => panic!("Expected vacant entry, but found occupied entry"),
    }

    // Test inserting a null value into a vacant entry
    match map.entry("key3") {
        Entry::Vacant(vacant) => {
            let value_ref: &mut Value = vacant.insert(Value::Null);
            assert_eq!(*value_ref, Value::Null);
            assert_eq!(map.get("key3"), Some(&Value::Null));
        }
        Entry::Occupied(_) => panic!("Expected vacant entry, but found occupied entry"),
    }
}

