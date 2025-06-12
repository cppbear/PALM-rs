// Answer 0

#[test]
fn test_serialize_i128_positive() {
    let value: i128 = 1234567890123456789;
    let result = serialize_i128(value);
    assert_eq!(result, Ok("1234567890123456789".to_string()));
}

#[test]
fn test_serialize_i128_negative() {
    let value: i128 = -1234567890123456789;
    let result = serialize_i128(value);
    assert_eq!(result, Ok("-1234567890123456789".to_string()));
}

#[test]
fn test_serialize_i128_zero() {
    let value: i128 = 0;
    let result = serialize_i128(value);
    assert_eq!(result, Ok("0".to_string()));
}

#[test]
#[should_panic]
fn test_serialize_i128_boundary() {
    let value: i128 = i128::MIN;
    let _ = serialize_i128(value); // This should not panic, so let's check if we can handle MIN correctly
}

#[test]
#[should_panic]
fn test_serialize_i128_max_boundary() {
    let value: i128 = i128::MAX;
    let _ = serialize_i128(value); // This should also work without triggering a panic
}

