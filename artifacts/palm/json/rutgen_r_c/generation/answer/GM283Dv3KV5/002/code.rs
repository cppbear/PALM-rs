// Answer 0

#[test]
fn test_as_f64_with_positive_number() {
    let number = Number::from_f64(256.0).unwrap();
    let value = Value::Number(number);
    assert_eq!(value.as_f64(), Some(256.0));
}

#[test]
fn test_as_f64_with_negative_number() {
    let number = Number::from_f64(-64.0).unwrap();
    let value = Value::Number(number);
    assert_eq!(value.as_f64(), Some(-64.0));
}

#[test]
fn test_as_f64_with_zero() {
    let number = Number::from_f64(0.0).unwrap();
    let value = Value::Number(number);
    assert_eq!(value.as_f64(), Some(0.0));
}

#[test]
fn test_as_f64_with_integer() {
    let number = Number::from_i128(64).unwrap();
    let value = Value::Number(number);
    assert_eq!(value.as_f64(), Some(64.0));
}

#[test]
fn test_as_f64_with_non_number_value() {
    let value = Value::Bool(true);
    assert_eq!(value.as_f64(), None);

    let value = Value::Null;
    assert_eq!(value.as_f64(), None);

    let value = Value::String(String::from("string"));
    assert_eq!(value.as_f64(), None);

    let value = Value::Array(vec![]);
    assert_eq!(value.as_f64(), None);

    let value = Value::Object(Map { map: MapImpl::new() });
    assert_eq!(value.as_f64(), None);
}

