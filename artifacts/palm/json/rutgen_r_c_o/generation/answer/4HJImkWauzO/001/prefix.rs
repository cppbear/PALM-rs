// Answer 0

#[test]
fn test_new_with_non_empty_object() {
    let mut map = Map::new();
    map.insert("key1".to_owned(), Value::Bool(true));
    let deserializer = MapDeserializer::new(map);
}

#[test]
fn test_new_with_string_value() {
    let mut map = Map::new();
    map.insert("key2".to_owned(), Value::String("test".to_owned()));
    let deserializer = MapDeserializer::new(map);
}

#[test]
fn test_new_with_number_value() {
    let mut map = Map::new();
    map.insert("key3".to_owned(), Value::Number(Number::from(42)));
    let deserializer = MapDeserializer::new(map);
}

#[test]
fn test_new_with_null_value() {
    let mut map = Map::new();
    map.insert("key4".to_owned(), Value::Null);
    let deserializer = MapDeserializer::new(map);
}

#[test]
fn test_new_with_array_value() {
    let mut map = Map::new();
    map.insert("key5".to_owned(), Value::Array(vec![Value::String("element1".to_owned()), Value::String("element2".to_owned())]));
    let deserializer = MapDeserializer::new(map);
}

#[test]
fn test_new_with_nested_object() {
    let mut inner_map = Map::new();
    inner_map.insert("inner_key".to_owned(), Value::Number(Number::from(3.14)));
    
    let mut map = Map::new();
    map.insert("key6".to_owned(), Value::Object(inner_map));
    let deserializer = MapDeserializer::new(map);
}

