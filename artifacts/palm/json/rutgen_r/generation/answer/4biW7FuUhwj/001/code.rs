// Answer 0

#[test]
fn test_is_number_with_non_number_value() {
    use serde_json::Value;

    // Testing with a boolean value
    let boolean_value = Value::Bool(true);
    assert!(!boolean_value.is_number());

    // Testing with a string value
    let string_value = Value::String("not a number".to_string());
    assert!(!string_value.is_number());

    // Testing with an array
    let array_value = Value::Array(vec![Value::Number(serde_json::Number::from(1))]);
    assert!(!array_value.is_number());

    // Testing with an object
    let object_value = Value::Object(serde_json::map::Map::new());
    assert!(!object_value.is_number());

    // Testing with a null value
    let null_value = Value::Null;
    assert!(!null_value.is_number());
}

