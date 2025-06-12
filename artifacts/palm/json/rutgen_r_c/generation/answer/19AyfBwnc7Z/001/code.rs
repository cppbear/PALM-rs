// Answer 0

#[test]
fn test_serialize_i16_positive() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_i16(42);
    assert_eq!(result, Ok("42".to_owned()));
}

#[test]
fn test_serialize_i16_negative() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_i16(-42);
    assert_eq!(result, Ok("-42".to_owned()));
}

#[test]
fn test_serialize_i16_zero() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_i16(0);
    assert_eq!(result, Ok("0".to_owned()));
}

#[test]
fn test_serialize_i16_min_value() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_i16(i16::MIN);
    assert_eq!(result, Ok(i16::MIN.to_string()));
}

#[test]
fn test_serialize_i16_max_value() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_i16(i16::MAX);
    assert_eq!(result, Ok(i16::MAX.to_string()));
}

