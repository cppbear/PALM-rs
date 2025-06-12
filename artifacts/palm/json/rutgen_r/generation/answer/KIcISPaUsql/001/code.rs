// Answer 0

#[test]
fn test_serialize_u64_zero() {
    let result = serialize_u64(0);
    assert_eq!(result, Ok("0".to_owned()));
}

#[test]
fn test_serialize_u64_small_value() {
    let result = serialize_u64(123);
    assert_eq!(result, Ok("123".to_owned()));
}

#[test]
fn test_serialize_u64_large_value() {
    let result = serialize_u64(4567890123);
    assert_eq!(result, Ok("4567890123".to_owned()));
}

#[test]
fn test_serialize_u64_max_value() {
    let result = serialize_u64(u64::MAX);
    assert_eq!(result, Ok("18446744073709551615".to_owned()));
}

