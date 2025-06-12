// Answer 0

#[test]
fn test_serialize_i8_positive_value() {
    let value: i8 = 42;
    let result = serialize_i8(value);
    assert_eq!(result, Ok("42".to_string()));
}

#[test]
fn test_serialize_i8_negative_value() {
    let value: i8 = -42;
    let result = serialize_i8(value);
    assert_eq!(result, Ok("-42".to_string()));
}

#[test]
fn test_serialize_i8_zero() {
    let value: i8 = 0;
    let result = serialize_i8(value);
    assert_eq!(result, Ok("0".to_string()));
}

#[test]
fn test_serialize_i8_boundary_positive() {
    let value: i8 = std::i8::MAX;
    let result = serialize_i8(value);
    assert_eq!(result, Ok("127".to_string()));
}

#[test]
fn test_serialize_i8_boundary_negative() {
    let value: i8 = std::i8::MIN;
    let result = serialize_i8(value);
    assert_eq!(result, Ok("-128".to_string()));
}

