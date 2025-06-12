// Answer 0

#[test]
fn test_as_object_with_single_entry() {
    let mut map = Map::new();
    map.insert(String::from("key1"), Value::Bool(true));
    let value = Value::Object(map);
    value.as_object();
}

#[test]
fn test_as_object_with_multiple_entries() {
    let mut map = Map::new();
    map.insert(String::from("key1"), Value::Number(Number { n: 42 }));
    map.insert(String::from("key2"), Value::String(String::from("value2")));
    let value = Value::Object(map);
    value.as_object();
}

#[test]
fn test_as_object_with_nested_object() {
    let mut nested_map = Map::new();
    nested_map.insert(String::from("nested_key"), Value::Bool(false));
    let mut map = Map::new();
    map.insert(String::from("outer_key"), Value::Object(nested_map));
    let value = Value::Object(map);
    value.as_object();
}

#[test]
fn test_as_object_with_empty_object() {
    let map = Map::new();
    let value = Value::Object(map);
    value.as_object();
}

#[test]
fn test_as_object_with_various_types() {
    let mut map = Map::new();
    map.insert(String::from("boolean"), Value::Bool(true));
    map.insert(String::from("number"), Value::Number(Number { n: 3.14 }));
    map.insert(String::from("string"), Value::String(String::from("text")));
    map.insert(String::from("array"), Value::Array(vec![Value::Null]));
    let value = Value::Object(map);
    value.as_object();
}

