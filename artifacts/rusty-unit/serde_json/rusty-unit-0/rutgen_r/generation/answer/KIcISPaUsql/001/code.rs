// Answer 0

#[test]
fn test_serialize_u64_zero() {
    let result = serialize_u64(0);
    assert_eq!(result, Ok("0".to_string()));
}

#[test]
fn test_serialize_u64_small_positive() {
    let result = serialize_u64(42);
    assert_eq!(result, Ok("42".to_string()));
}

#[test]
fn test_serialize_u64_large_value() {
    let result = serialize_u64(1_000_000_000_000);
    assert_eq!(result, Ok("1000000000000".to_string()));
}

#[test]
fn test_serialize_u64_max_value() {
    let result = serialize_u64(u64::MAX);
    assert_eq!(result, Ok("18446744073709551615".to_string()));
}

#[test]
fn test_serialize_u64_one() {
    let result = serialize_u64(1);
    assert_eq!(result, Ok("1".to_string()));
}

