// Answer 0

#[test]
fn test_serialize_i32_positive_value() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_i32(42);
    assert_eq!(result, Ok("42".to_owned()));
}

#[test]
fn test_serialize_i32_negative_value() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_i32(-42);
    assert_eq!(result, Ok("-42".to_owned()));
}

#[test]
fn test_serialize_i32_zero_value() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_i32(0);
    assert_eq!(result, Ok("0".to_owned()));
}

#[test]
fn test_serialize_i32_max_value() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_i32(i32::MAX);
    assert_eq!(result, Ok("2147483647".to_owned()));
}

#[test]
fn test_serialize_i32_min_value() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_i32(i32::MIN);
    assert_eq!(result, Ok("-2147483648".to_owned()));
}

