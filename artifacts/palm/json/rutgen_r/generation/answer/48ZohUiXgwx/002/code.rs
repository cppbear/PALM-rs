// Answer 0

#[test]
fn test_or_insert_vacant_entry() {
    use serde_json::{Map, Value};

    let mut map = Map::new();
    let default_value = Value::from(42);

    let entry = map.entry("nonexistent_key");
    let result = entry.or_insert(default_value.clone());

    assert_eq!(map["nonexistent_key"], default_value);
    assert_eq!(result, &mut map["nonexistent_key"]);
}

#[test]
fn test_or_insert_occupied_entry() {
    use serde_json::{Map, Value};

    let mut map = Map::new();
    map.insert("existing_key".to_string(), Value::from(100));
    
    let default_value = Value::from(200);

    let entry = map.entry("existing_key");
    let result = entry.or_insert(default_value.clone());

    assert_eq!(map["existing_key"], Value::from(100));
    assert_eq!(result, &mut map["existing_key"]);
}

