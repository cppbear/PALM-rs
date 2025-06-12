// Answer 0

#[test]
fn test_get_key_value_existing_key() {
    use std::collections::HashMap;
    use serde_json::Value;

    let mut map: HashMap<String, Value> = HashMap::new();
    map.insert("key1".to_string(), Value::from(1));
    map.insert("key2".to_string(), Value::from(2));

    let result = map.get_key_value(&"key1");
    assert_eq!(result, Some((&"key1".to_string(), &Value::from(1))));
}

#[test]
fn test_get_key_value_non_existing_key() {
    use std::collections::HashMap;
    use serde_json::Value;

    let mut map: HashMap<String, Value> = HashMap::new();
    map.insert("key1".to_string(), Value::from(1));
    map.insert("key2".to_string(), Value::from(2));

    let result = map.get_key_value(&"key3");
    assert_eq!(result, None);
}

#[test]
fn test_get_key_value_with_different_borrowed_type() {
    use std::collections::HashMap;
    use serde_json::Value;

    let mut map: HashMap<String, Value> = HashMap::new();
    map.insert("key1".to_string(), Value::from(1));
    
    let result = map.get_key_value(&String::from("key1"));
    assert_eq!(result, Some((&"key1".to_string(), &Value::from(1))));
}

#[test]
fn test_get_key_value_empty_map() {
    use std::collections::HashMap;
    use serde_json::Value;

    let map: HashMap<String, Value> = HashMap::new();

    let result = map.get_key_value(&"key1");
    assert_eq!(result, None);
}

#[test]
fn test_get_key_value_key_with_mixed_case() {
    use std::collections::HashMap;
    use serde_json::Value;

    let mut map: HashMap<String, Value> = HashMap::new();
    map.insert("Key1".to_string(), Value::from(1));
    
    let result = map.get_key_value(&"key1");
    assert_eq!(result, None);
}

