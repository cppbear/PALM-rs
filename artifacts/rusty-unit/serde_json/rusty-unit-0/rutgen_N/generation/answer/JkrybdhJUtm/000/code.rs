// Answer 0

#[test]
fn test_or_insert_with_vacant_entry() {
    use serde_json::{json, Map, Value};

    let mut map = Map::new();
    let value = map.entry("key").or_insert_with(|| json!("default_value"));
    
    assert_eq!(value, &json!("default_value"));
    assert_eq!(map["key"], json!("default_value"));
}

#[test]
fn test_or_insert_with_occupied_entry() {
    use serde_json::{json, Map, Value};

    let mut map = Map::new();
    map.insert("key".to_owned(), json!("original_value"));

    let value = map.entry("key").or_insert_with(|| json!("default_value"));
    
    assert_eq!(value, &json!("original_value"));
    assert_eq!(map["key"], json!("original_value"));
}

#[test]
fn test_or_insert_with_edge_case() {
    use serde_json::{json, Map, Value};

    let mut map = Map::new();
    
    // Inserting multiple entries to see if it maintains correct value
    let first_value = map.entry("key").or_insert_with(|| json!("value1"));
    assert_eq!(first_value, &json!("value1"));
    
    let second_value = map.entry("key").or_insert_with(|| json!("value2"));
    assert_eq!(second_value, &json!("value1")); // Should still return "value1"

    assert_eq!(map["key"], json!("value1"));
}

