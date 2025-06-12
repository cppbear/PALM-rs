// Answer 0

#[test]
fn test_serialize_i8_positive_value() {
    let result = serialize_i8(42);
    assert_eq!(result, Ok("42".to_string()));
}

#[test]
fn test_serialize_i8_negative_value() {
    let result = serialize_i8(-42);
    assert_eq!(result, Ok("-42".to_string()));
}

#[test]
fn test_serialize_i8_zero_value() {
    let result = serialize_i8(0);
    assert_eq!(result, Ok("0".to_string()));
}

#[test]
fn test_serialize_i8_min_value() {
    let result = serialize_i8(i8::MIN);
    assert_eq!(result, Ok(i8::MIN.to_string()));
}

#[test]
fn test_serialize_i8_max_value() {
    let result = serialize_i8(i8::MAX);
    assert_eq!(result, Ok(i8::MAX.to_string()));
}

