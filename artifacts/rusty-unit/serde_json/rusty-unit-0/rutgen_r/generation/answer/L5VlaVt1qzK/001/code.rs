// Answer 0

#[test]
fn test_new_with_empty_map() {
    use std::collections::HashMap;
    use serde_json::Value;
    use serde_json::de::MapRefDeserializer;

    let map: HashMap<String, Value> = HashMap::new();
    let deserializer = MapRefDeserializer::new(&map);
    
    assert_eq!(deserializer.iter.len(), 0);
    assert_eq!(deserializer.value, None);
}

#[test]
fn test_new_with_single_element_map() {
    use std::collections::HashMap;
    use serde_json::Value;
    use serde_json::de::MapRefDeserializer;

    let mut map = HashMap::new();
    map.insert("key1".to_string(), Value::from("value1"));
    let deserializer = MapRefDeserializer::new(&map);
    
    assert_eq!(deserializer.iter.len(), 1);
    assert_eq!(deserializer.value, None);
}

#[test]
fn test_new_with_multiple_elements_map() {
    use std::collections::HashMap;
    use serde_json::Value;
    use serde_json::de::MapRefDeserializer;

    let mut map = HashMap::new();
    map.insert("key1".to_string(), Value::from("value1"));
    map.insert("key2".to_string(), Value::from("value2"));
    let deserializer = MapRefDeserializer::new(&map);

    assert_eq!(deserializer.iter.len(), 2);
    assert_eq!(deserializer.value, None);
}

#[test]
fn test_new_with_non_string_key() {
    use std::collections::HashMap;
    use serde_json::Value;
    use serde_json::de::MapRefDeserializer;

    let mut map: HashMap<String, Value> = HashMap::new();
    map.insert("key1".to_string(), Value::from("value1"));
    map.insert("key2".to_string(), Value::from(2));
    let deserializer = MapRefDeserializer::new(&map);

    assert_eq!(deserializer.iter.len(), 2);
    assert_eq!(deserializer.value, None);
}

