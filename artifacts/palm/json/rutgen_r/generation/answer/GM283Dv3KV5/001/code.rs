// Answer 0

#[test]
fn test_as_f64_with_non_number_value() {
    use serde_json::Value;

    let null_value = Value::Null;
    let string_value = Value::String("not a number".to_string());
    let array_value = Value::Array(vec![Value::Number(1.into())]);
    let object_value = Value::Object(serde_json::map::Map::new());

    assert_eq!(null_value.as_f64(), None);
    assert_eq!(string_value.as_f64(), None);
    assert_eq!(array_value.as_f64(), None);
    assert_eq!(object_value.as_f64(), None);
}

#[test]
fn test_as_f64_with_boundary_conditions() {
    use serde_json::Value;

    let min_value = Value::Number(serde_json::Number::from_f64(f64::MIN).unwrap());
    let max_value = Value::Number(serde_json::Number::from_f64(f64::MAX).unwrap());
    let subnormal_value = Value::Number(serde_json::Number::from_f64(f64::MIN_POSITIVE).unwrap());

    assert_eq!(min_value.as_f64(), Some(f64::MIN));
    assert_eq!(max_value.as_f64(), Some(f64::MAX));
    assert_eq!(subnormal_value.as_f64(), Some(f64::MIN_POSITIVE));
}

