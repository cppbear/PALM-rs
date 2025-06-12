// Answer 0

#[test]
fn test_serialize_i64_positive() {
    let value: i64 = 42;
    let serialized = serialize_i64(value).unwrap();
    assert_eq!(serialized, "42");
}

#[test]
fn test_serialize_i64_negative() {
    let value: i64 = -42;
    let serialized = serialize_i64(value).unwrap();
    assert_eq!(serialized, "-42");
}

#[test]
fn test_serialize_i64_zero() {
    let value: i64 = 0;
    let serialized = serialize_i64(value).unwrap();
    assert_eq!(serialized, "0");
}

#[test]
fn test_serialize_i64_large() {
    let value: i64 = 1_000_000_000_000;
    let serialized = serialize_i64(value).unwrap();
    assert_eq!(serialized, "1000000000000");
}

#[test]
fn test_serialize_i64_small() {
    let value: i64 = -1_000_000_000_000;
    let serialized = serialize_i64(value).unwrap();
    assert_eq!(serialized, "-1000000000000");
}

