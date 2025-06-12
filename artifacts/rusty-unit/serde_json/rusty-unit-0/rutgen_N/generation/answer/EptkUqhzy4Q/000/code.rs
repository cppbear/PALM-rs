// Answer 0

#[test]
fn test_serialize_i8_positive_value() {
    let value: i8 = 42;
    let result = serialize_i8(value);
    assert_eq!(result.unwrap(), "42");
}

#[test]
fn test_serialize_i8_negative_value() {
    let value: i8 = -42;
    let result = serialize_i8(value);
    assert_eq!(result.unwrap(), "-42");
}

#[test]
fn test_serialize_i8_zero() {
    let value: i8 = 0;
    let result = serialize_i8(value);
    assert_eq!(result.unwrap(), "0");
}

