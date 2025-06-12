// Answer 0

#[test]
fn test_serialize_i8_positive_value() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_i8(42);
    assert_eq!(result, Ok("42".to_owned()));
}

#[test]
fn test_serialize_i8_negative_value() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_i8(-15);
    assert_eq!(result, Ok("-15".to_owned()));
}

#[test]
fn test_serialize_i8_zero_value() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_i8(0);
    assert_eq!(result, Ok("0".to_owned()));
}

