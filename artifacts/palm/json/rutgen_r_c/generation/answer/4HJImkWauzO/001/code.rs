// Answer 0

#[test]
fn test_new_with_empty_map() {
    let map = Map { map: std::collections::BTreeMap::new() };
    let deserializer = MapDeserializer::new(map);
    assert!(deserializer.value.is_none());
    assert_eq!(deserializer.iter.len(), 0);
}

#[test]
fn test_new_with_single_entry_map() {
    let mut btree_map = std::collections::BTreeMap::new();
    btree_map.insert("key1".to_string(), Value::String("value1".to_string()));
    let map = Map { map: btree_map };
    let deserializer = MapDeserializer::new(map);
    assert!(deserializer.value.is_none());
    assert_eq!(deserializer.iter.len(), 1);
}

#[test]
fn test_new_with_multiple_entries_map() {
    let mut btree_map = std::collections::BTreeMap::new();
    btree_map.insert("key1".to_string(), Value::Number(Number::from(10)));
    btree_map.insert("key2".to_string(), Value::Bool(true));
    let map = Map { map: btree_map };
    let deserializer = MapDeserializer::new(map);
    assert!(deserializer.value.is_none());
    assert_eq!(deserializer.iter.len(), 2);
}

#[test]
fn test_new_with_various_value_types() {
    let mut btree_map = std::collections::BTreeMap::new();
    btree_map.insert("null".to_string(), Value::Null);
    btree_map.insert("bool".to_string(), Value::Bool(false));
    btree_map.insert("number".to_string(), Value::Number(Number::from(3.14)));
    btree_map.insert("string".to_string(), Value::String("Hello".to_string()));
    btree_map.insert("array".to_string(), Value::Array(vec![Value::Number(Number::from(1)), Value::Number(Number::from(2))]));
    btree_map.insert("object".to_string(), Value::Object(Map { map: std::collections::BTreeMap::new() }));
    let map = Map { map: btree_map };
    let deserializer = MapDeserializer::new(map);
    assert!(deserializer.value.is_none());
    assert_eq!(deserializer.iter.len(), 6);
}

