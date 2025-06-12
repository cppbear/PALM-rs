// Answer 0

#[test]
fn test_as_i64_with_integer() {
    let v = Value::Number(Number::from_i128(64).unwrap());
    assert_eq!(v.as_i64(), Some(64));
}

#[test]
fn test_as_i64_with_large_integer() {
    let big = i64::MAX as u64 + 10;
    let v = Value::Number(Number::from_u128(big).unwrap());
    assert_eq!(v.as_i64(), None);
}

#[test]
fn test_as_i64_with_floating_point() {
    let v = Value::Number(Number::from_f64(256.0).unwrap());
    assert_eq!(v.as_i64(), None);
}

#[test]
fn test_as_i64_with_string() {
    let v = Value::String("not a number".to_string());
    assert_eq!(v.as_i64(), None);
}

#[test]
fn test_as_i64_with_null() {
    let v = Value::Null;
    assert_eq!(v.as_i64(), None);
}

