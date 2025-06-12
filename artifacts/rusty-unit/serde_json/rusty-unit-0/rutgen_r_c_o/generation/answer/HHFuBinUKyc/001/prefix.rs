// Answer 0

#[test]
fn test_index_null_value() {
    let value = Value::Null;
    let result = value["a"];
}

#[test]
fn test_index_bool_value() {
    let value = Value::Bool(true);
    let result = value["a"];
}

#[test]
fn test_index_array_within_bounds() {
    let value = Value::Array(vec![Value::String(String::from("element1")), Value::String(String::from("element2"))]);
    let result = value[1];
}

#[test]
fn test_index_array_out_of_bounds() {
    let value = Value::Array(vec![Value::String(String::from("element1")), Value::String(String::from("element2"))]);
    let result = value[2];
}

#[test]
fn test_index_object_key_exists() {
    let mut obj = Map::new();
    obj.insert(String::from("key1"), Value::String(String::from("value1")));
    let value = Value::Object(obj);
    let result = value["key1"];
}

#[test]
fn test_index_object_key_does_not_exist() {
    let mut obj = Map::new();
    obj.insert(String::from("key1"), Value::String(String::from("value1")));
    let value = Value::Object(obj);
    let result = value["key2"];
}

#[test]
fn test_index_object_with_invalid_type() {
    let mut obj = Map::new();
    obj.insert(String::from("key1"), Value::Number(Number { n: 42 }));
    let value = Value::Object(obj);
    let result = value["key1"][0];
}

#[test]
fn test_index_array_with_string_key() {
    let value = Value::Array(vec![Value::String(String::from("element1")), Value::String(String::from("element2"))]);
    let result = value["key"];
}

#[test]
fn test_index_object_with_array_value() {
    let mut obj = Map::new();
    obj.insert(String::from("numbers"), Value::Array(vec![Value::Number(Number { n: 42 }), Value::Number(Number { n: 43 })]));
    let value = Value::Object(obj);
    let result = value["numbers"][1];
}

#[test]
fn test_index_empty_object() {
    let obj = Map::new();
    let value = Value::Object(obj);
    let result = value["some_key"];
}

