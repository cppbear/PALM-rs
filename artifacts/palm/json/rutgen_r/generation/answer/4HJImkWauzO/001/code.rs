// Answer 0

#[test]
fn test_new_with_empty_map() {
    use serde_json::Map;
    use serde_json::Value;
    use serde_json::de::MapDeserializer;

    let map: Map<String, Value> = Map::new();
    let deserializer = MapDeserializer::new(map);

    assert!(deserializer.iter.next().is_none());
    assert!(deserializer.value.is_none());
}

#[test]
fn test_new_with_single_entry_map() {
    use serde_json::Map;
    use serde_json::Value;
    use serde_json::de::MapDeserializer;

    let mut map = Map::new();
    map.insert("key".to_string(), Value::String("value".to_string()));
    let deserializer = MapDeserializer::new(map);

    assert!(deserializer.iter.next().is_some());
    assert!(deserializer.value.is_none());
}

#[test]
fn test_new_with_multiple_entries_map() {
    use serde_json::Map;
    use serde_json::Value;
    use serde_json::de::MapDeserializer;

    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.insert("key2".to_string(), Value::Number(serde_json::Number::from(42)));
    let deserializer = MapDeserializer::new(map);

    assert_eq!(deserializer.iter.count(), 2);
    assert!(deserializer.value.is_none());
}

#[test]
fn test_new_with_non_string_keys() {
    use serde_json::Map;
    use serde_json::Value;
    use serde_json::de::MapDeserializer;

    let mut map = Map::new();
    map.insert("key".to_string(), Value::Array(vec![Value::Number(1.into()), Value::Number(2.into())]));
    let deserializer = MapDeserializer::new(map);

    assert!(deserializer.iter.next().is_some());
    assert!(deserializer.value.is_none());
}

