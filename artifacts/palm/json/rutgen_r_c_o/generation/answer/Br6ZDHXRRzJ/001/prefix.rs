// Answer 0

#[test]
fn test_unexpected_for_object_with_single_entry() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Null);
    let value = Value::Object(map);
    value.unexpected();
}

#[test]
fn test_unexpected_for_object_with_multiple_entries() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    map.insert("key2".to_string(), Value::Number(Number { n: 42 }));
    let value = Value::Object(map);
    value.unexpected();
}

#[test]
fn test_unexpected_for_object_with_string_value() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("some string".to_string()));
    let value = Value::Object(map);
    value.unexpected();
}

#[test]
fn test_unexpected_for_object_with_array_value() {
    let mut map = Map::new();
    let array_value = Value::Array(vec![Value::Bool(false), Value::String("item".to_string())]);
    map.insert("key1".to_string(), array_value);
    let value = Value::Object(map);
    value.unexpected();
}

