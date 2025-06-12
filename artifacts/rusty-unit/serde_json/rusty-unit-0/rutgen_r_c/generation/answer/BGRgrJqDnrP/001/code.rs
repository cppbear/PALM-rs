// Answer 0

#[test]
fn test_serialize_i64_positive() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_i64(42);
    assert_eq!(result, Ok("42".to_owned()));
}

#[test]
fn test_serialize_i64_zero() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_i64(0);
    assert_eq!(result, Ok("0".to_owned()));
}

#[test]
fn test_serialize_i64_negative() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_i64(-42);
    assert_eq!(result, Ok("-42".to_owned()));
}

#[test]
fn test_serialize_i64_min_value() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_i64(i64::MIN);
    assert_eq!(result, Ok(itoa::Buffer::new().format(i64::MIN).to_owned()));
}

#[test]
fn test_serialize_i64_max_value() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_i64(i64::MAX);
    assert_eq!(result, Ok(itoa::Buffer::new().format(i64::MAX).to_owned()));
}

