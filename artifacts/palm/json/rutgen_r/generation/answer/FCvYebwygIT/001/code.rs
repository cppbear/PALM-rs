// Answer 0

#[test]
fn test_serialize_i32_positive() {
    let value: i32 = 42;
    let result = serialize_i32(value);
    assert_eq!(result, Ok("42".to_string()));
}

#[test]
fn test_serialize_i32_negative() {
    let value: i32 = -42;
    let result = serialize_i32(value);
    assert_eq!(result, Ok("-42".to_string()));
}

#[test]
fn test_serialize_i32_zero() {
    let value: i32 = 0;
    let result = serialize_i32(value);
    assert_eq!(result, Ok("0".to_string()));
}

#[test]
fn test_serialize_i32_max() {
    let value: i32 = i32::MAX;
    let result = serialize_i32(value);
    assert_eq!(result, Ok("2147483647".to_string()));
}

#[test]
fn test_serialize_i32_min() {
    let value: i32 = i32::MIN;
    let result = serialize_i32(value);
    assert_eq!(result, Ok("-2147483648".to_string()));
}

