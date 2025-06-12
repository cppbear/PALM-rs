// Answer 0

#[test]
fn test_serialize_i32_positive() {
    let value: i32 = 42;
    let result = serialize_i32(value).unwrap();
    assert_eq!(result, "42");
}

#[test]
fn test_serialize_i32_negative() {
    let value: i32 = -42;
    let result = serialize_i32(value).unwrap();
    assert_eq!(result, "-42");
}

#[test]
fn test_serialize_i32_zero() {
    let value: i32 = 0;
    let result = serialize_i32(value).unwrap();
    assert_eq!(result, "0");
}

