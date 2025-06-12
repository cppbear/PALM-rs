// Answer 0

#[test]
fn test_as_f64_with_valid_numbers() {
    use serde_json::Value;

    let num_value_1 = Value::Number(serde_json::Number::from_f64(256.0).unwrap());
    let num_value_2 = Value::Number(serde_json::Number::from(64));
    let num_value_3 = Value::Number(serde_json::Number::from(-64));

    assert_eq!(num_value_1.as_f64(), Some(256.0));
    assert_eq!(num_value_2.as_f64(), Some(64.0));
    assert_eq!(num_value_3.as_f64(), Some(-64.0));
}

#[test]
fn test_as_f64_with_non_number_values() {
    use serde_json::Value;

    let string_value = Value::String("not a number".to_string());
    let boolean_value = Value::Bool(true);
    let array_value = Value::Array(vec![Value::Number(serde_json::Number::from(1))]);
    let object_value = Value::Object(serde_json::map::Map::new());

    assert_eq!(string_value.as_f64(), None);
    assert_eq!(boolean_value.as_f64(), None);
    assert_eq!(array_value.as_f64(), None);
    assert_eq!(object_value.as_f64(), None);
}

#[test]
fn test_as_f64_with_edge_cases() {
    use serde_json::Value;

    let zero_value = Value::Number(serde_json::Number::from(0));
    let very_large_value = Value::Number(serde_json::Number::from_f64(1.7976931348623157e+308).unwrap()); // max f64
    let very_small_value = Value::Number(serde_json::Number::from_f64(-1.7976931348623157e+308).unwrap()); // min f64
    
    assert_eq!(zero_value.as_f64(), Some(0.0));
    assert_eq!(very_large_value.as_f64(), Some(1.7976931348623157e+308));
    assert_eq!(very_small_value.as_f64(), Some(-1.7976931348623157e+308));
}

