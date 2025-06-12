// Answer 0

#[test]
fn test_serialize_u8_zero() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_u8(0);
    assert_eq!(result, Ok("0".to_owned()));
}

#[test]
fn test_serialize_u8_max() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_u8(u8::MAX);
    assert_eq!(result, Ok("255".to_owned()));
}

#[test]
fn test_serialize_u8_min() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_u8(u8::MIN);
    assert_eq!(result, Ok("0".to_owned()));
}

#[test]
fn test_serialize_u8_random() {
    let serializer = MapKeySerializer;
    let random_values = [7, 128, 255];
    for &value in &random_values {
        let result = serializer.serialize_u8(value);
        assert_eq!(result, Ok(itoa::Buffer::new().format(value).to_owned()));
    }
}

