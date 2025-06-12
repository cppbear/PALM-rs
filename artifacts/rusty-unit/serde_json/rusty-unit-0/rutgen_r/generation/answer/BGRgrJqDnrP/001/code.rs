// Answer 0

#[test]
fn test_serialize_i64_positive_value() {
    let value: i64 = 12345;
    let result = serialize_i64(value);
    assert_eq!(result, Ok("12345".to_string()));
}

#[test]
fn test_serialize_i64_negative_value() {
    let value: i64 = -12345;
    let result = serialize_i64(value);
    assert_eq!(result, Ok("-12345".to_string()));
}

#[test]
fn test_serialize_i64_zero() {
    let value: i64 = 0;
    let result = serialize_i64(value);
    assert_eq!(result, Ok("0".to_string()));
}

#[test]
fn test_serialize_i64_boundary_positive() {
    let value: i64 = i64::MAX;
    let result = serialize_i64(value);
    assert_eq!(result, Ok("9223372036854775807".to_string()));
}

#[test]
fn test_serialize_i64_boundary_negative() {
    let value: i64 = i64::MIN;
    let result = serialize_i64(value);
    assert_eq!(result, Ok("-9223372036854775808".to_string()));
}

