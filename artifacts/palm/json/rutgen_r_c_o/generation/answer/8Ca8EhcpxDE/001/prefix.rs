// Answer 0

#[test]
fn test_end_with_non_empty_name_and_null_value() {
    let name = String::from("variant1");
    let mut map = Map::new();
    map.insert(String::from("key1"), Value::Null);
    let variant = SerializeStructVariant { name, map };
    let _ = variant.end();
}

#[test]
fn test_end_with_non_empty_name_and_bool_value() {
    let name = String::from("variant2");
    let mut map = Map::new();
    map.insert(String::from("key2"), Value::Bool(true));
    let variant = SerializeStructVariant { name, map };
    let _ = variant.end();
}

#[test]
fn test_end_with_non_empty_name_and_number_value() {
    let name = String::from("variant3");
    let mut map = Map::new();
    map.insert(String::from("key3"), Value::Number(Number::from(42)));
    let variant = SerializeStructVariant { name, map };
    let _ = variant.end();
}

#[test]
fn test_end_with_non_empty_name_and_string_value() {
    let name = String::from("variant4");
    let mut map = Map::new();
    map.insert(String::from("key4"), Value::String(String::from("value")));
    let variant = SerializeStructVariant { name, map };
    let _ = variant.end();
}

#[test]
fn test_end_with_non_empty_name_and_array_value() {
    let name = String::from("variant5");
    let mut map = Map::new();
    map.insert(String::from("key5"), Value::Array(vec![Value::String(String::from("item1")), Value::String(String::from("item2"))]));
    let variant = SerializeStructVariant { name, map };
    let _ = variant.end();
}

#[test]
fn test_end_with_non_empty_name_and_object_value() {
    let name = String::from("variant6");
    let mut inner_map = Map::new();
    inner_map.insert(String::from("inner_key"), Value::Bool(false));
    let mut map = Map::new();
    map.insert(String::from("key6"), Value::Object(inner_map));
    let variant = SerializeStructVariant { name, map };
    let _ = variant.end();
}

#[test]
fn test_end_with_maximum_length_name() {
    let name = String::from("a".repeat(255));
    let mut map = Map::new();
    map.insert(String::from("key_max"), Value::String(String::from("max_length_test")));
    let variant = SerializeStructVariant { name, map };
    let _ = variant.end();
}

