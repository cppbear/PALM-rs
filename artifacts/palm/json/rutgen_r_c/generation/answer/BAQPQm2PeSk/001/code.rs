// Answer 0

#[test]
fn test_is_object_with_null() {
    let value = Value::Null;
    assert!(!value.is_object());
}

#[test]
fn test_is_object_with_bool() {
    let value = Value::Bool(true);
    assert!(!value.is_object());
}

#[test]
fn test_is_object_with_number() {
    let value = Value::Number(Number { n: 0 }); // assuming N = i32 for this example
    assert!(!value.is_object());
}

#[test]
fn test_is_object_with_string() {
    let value = Value::String(String::from("a string"));
    assert!(!value.is_object());
}

#[test]
fn test_is_object_with_array() {
    let value = Value::Array(vec![
        Value::String(String::from("an")),
        Value::String(String::from("array")),
    ]);
    assert!(!value.is_object());
}

#[test]
fn test_is_object_with_object() {
    let mut map = Map { map: MapImpl::new() }; // assuming MapImpl has a new() method
    map.insert(String::from("key"), Value::String(String::from("value"))); // assuming insert method
    let value = Value::Object(map);
    assert!(value.is_object());
}

#[test]
fn test_is_object_with_nested_object() {
    let mut inner_map = Map { map: MapImpl::new() };
    inner_map.insert(String::from("nested"), Value::Bool(true)); // assuming insert
    
    let mut outer_map = Map { map: MapImpl::new() };
    outer_map.insert(String::from("a"), Value::Object(inner_map));
    outer_map.insert(String::from("b"), Value::Array(vec![])); // assuming empty array
    
    let value = Value::Object(outer_map);
    assert!(value.is_object());
    if let Some(inner) = value.as_object() {
        if let Some(nested) = inner.get(&String::from("a")) {
            assert!(nested.is_object());
        }
    }
    
    if let Some(non_object) = value.as_object() {
        assert!(!non_object.get(&String::from("b")).unwrap().is_object());
    }
}

