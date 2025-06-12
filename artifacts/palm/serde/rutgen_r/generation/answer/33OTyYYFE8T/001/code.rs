// Answer 0

#[derive(Debug)]
struct Content {
    value: i8,
}

impl Content {
    fn I8(v: i8) -> Self {
        Content { value: v }
    }
}

struct Serializer;

impl Serializer {
    fn serialize_i8(self, v: i8) -> Result<Content, &'static str> {
        Ok(Content::I8(v))
    }
}

#[test]
fn test_serialize_i8_positive() {
    let serializer = Serializer;
    let result = serializer.serialize_i8(42);
    assert_eq!(result, Ok(Content::I8(42)));
}

#[test]
fn test_serialize_i8_negative() {
    let serializer = Serializer;
    let result = serializer.serialize_i8(-42);
    assert_eq!(result, Ok(Content::I8(-42)));
}

#[test]
fn test_serialize_i8_zero() {
    let serializer = Serializer;
    let result = serializer.serialize_i8(0);
    assert_eq!(result, Ok(Content::I8(0)));
}

#[test]
fn test_serialize_i8_boundary_positive() {
    let serializer = Serializer;
    let result = serializer.serialize_i8(i8::MAX);
    assert_eq!(result, Ok(Content::I8(i8::MAX)));
}

#[test]
fn test_serialize_i8_boundary_negative() {
    let serializer = Serializer;
    let result = serializer.serialize_i8(i8::MIN);
    assert_eq!(result, Ok(Content::I8(i8::MIN)));
}

