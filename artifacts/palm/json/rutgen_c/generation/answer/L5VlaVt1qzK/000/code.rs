// Answer 0

#[test]
fn test_map_ref_deserializer_new() {
    use std::collections::BTreeMap;
    use serde_json::Value;

    let mut map = BTreeMap::new();
    map.insert("key1".to_string(), Value::Bool(true));
    map.insert("key2".to_string(), Value::Number(serde_json::Number::from(42)));

    let serde_map = serde_json::map::Map::from(map);
    let deserializer = MapRefDeserializer::new(&serde_map);

    assert_eq!(deserializer.iter.len(), 2);
    assert!(deserializer.value.is_none());
}

#[test]
fn test_map_ref_deserializer_new_empty() {
    use std::collections::BTreeMap;
    use serde_json::Value;

    let map = BTreeMap::new();
    let serde_map = serde_json::map::Map::from(map);
    let deserializer = MapRefDeserializer::new(&serde_map);

    assert_eq!(deserializer.iter.len(), 0);
    assert!(deserializer.value.is_none());
}

