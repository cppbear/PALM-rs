// Answer 0

#[test]
fn test_is_number_with_number_value() {
    use serde_json::Value;

    let number_value = Value::Number(serde_json::Number::from(42));
    assert!(number_value.is_number());
}

#[test]
fn test_is_number_with_negative_number_value() {
    use serde_json::Value;

    let negative_number_value = Value::Number(serde_json::Number::from(-42));
    assert!(negative_number_value.is_number());
}

#[test]
fn test_is_number_with_float_value() {
    use serde_json::Value;

    let float_value = Value::Number(serde_json::Number::from_f64(3.14).unwrap());
    assert!(float_value.is_number());
}

