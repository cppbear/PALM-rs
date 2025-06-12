// Answer 0

#[test]
fn test_value_is_null() {
    use serde_json::Value;

    // Test for a Value that is explicitly null
    let null_value = Value::Null;
    assert!(null_value.is_null());

    // Test for a Value that is a boolean (not null)
    let bool_value = Value::Bool(false);
    assert!(!bool_value.is_null());

    // Test for a Value that is a number (not null)
    let num_value = Value::Number(Number { n: 10.0 });
    assert!(!num_value.is_null());

    // Test for a Value that is a string (not null)
    let string_value = Value::String(String::from("a string"));
    assert!(!string_value.is_null());

    // Test for a Value that is an array (not null)
    let array_value = Value::Array(vec![Value::null(), Value::Bool(true)]);
    assert!(!array_value.is_null());

    // Test for a Value that is an object (not null)
    let object_value = Value::Object(Map { map: vec![] });
    assert!(!object_value.is_null());
}

#[test]
fn test_null_is_null() {
    use serde_json::Value;

    // Create a JSON object with null value
    let json_object = Value::Object(vec![("a".to_string(), Value::Null)].into_iter().collect());
    assert!(json_object.get(&"a".to_string()).unwrap().is_null());
}

#[test]
fn test_empty_value_is_not_null() {
    use serde_json::Value;

    // Create an empty Value
    let empty_value = Value::Bool(false);
    assert!(!empty_value.is_null());
}

