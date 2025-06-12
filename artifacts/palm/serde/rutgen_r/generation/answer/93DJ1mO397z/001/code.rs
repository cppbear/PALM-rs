// Answer 0

#[derive(Debug)]
struct Content {
    value: i16,
}

impl Content {
    fn I16(v: i16) -> Self {
        Content { value: v }
    }
}

struct Serializer;

impl Serializer {
    fn serialize_i16(self, v: i16) -> Result<Content, &'static str> {
        Ok(Content::I16(v))
    }
}

#[test]
fn test_serialize_i16_positive() {
    let serializer = Serializer;
    let result = serializer.serialize_i16(10);
    assert_eq!(result.unwrap().value, 10);
}

#[test]
fn test_serialize_i16_negative() {
    let serializer = Serializer;
    let result = serializer.serialize_i16(-10);
    assert_eq!(result.unwrap().value, -10);
}

#[test]
fn test_serialize_i16_zero() {
    let serializer = Serializer;
    let result = serializer.serialize_i16(0);
    assert_eq!(result.unwrap().value, 0);
}

#[test]
fn test_serialize_i16_max() {
    let serializer = Serializer;
    let result = serializer.serialize_i16(i16::MAX);
    assert_eq!(result.unwrap().value, i16::MAX);
}

#[test]
fn test_serialize_i16_min() {
    let serializer = Serializer;
    let result = serializer.serialize_i16(i16::MIN);
    assert_eq!(result.unwrap().value, i16::MIN);
}

