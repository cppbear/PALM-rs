// Answer 0

#[test]
fn test_or_insert_with_vacant_entry() {
    use serde_json::{Map, json, Value};
    use std::collections::hash_map::Entry;

    let mut map: Map<String, Value> = Map::new();
    let default_value = json!(42);

    let entry = map.entry("example_key".to_string());
    let value_ref = match entry {
        Entry::Vacant(v) => v.or_insert(default_value),
        Entry::Occupied(_) => panic!("Expected a vacant entry"),
    };

    assert_eq!(*value_ref, 42);
    assert_eq!(map["example_key"], 42);
}

#[test]
fn test_or_insert_with_occupied_entry() {
    use serde_json::{Map, json, Value};
    use std::collections::hash_map::Entry;

    let mut map: Map<String, Value> = Map::new();
    map.insert("key".to_string(), json!(10));
    let default_value = json!(20);

    let entry = map.entry("key".to_string());
    let value_ref = match entry {
        Entry::Vacant(_) => panic!("Expected an occupied entry"),
        Entry::Occupied(o) => o.or_insert(default_value),
    };

    assert_eq!(*value_ref, 10);
    assert_eq!(map["key"], 10);
}

