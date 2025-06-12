// Answer 0

#[derive(Debug)]
struct Content {
    value: f32,
}

impl Content {
    fn F32(v: f32) -> Self {
        Content { value: v }
    }
}

struct Serializer;

impl Serializer {
    fn serialize_f32(self, v: f32) -> Result<Content, &'static str> {
        Ok(Content::F32(v))
    }
}

#[test]
fn test_serialize_f32_positive() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(3.14);
    assert_eq!(result, Ok(Content::F32(3.14)));
}

#[test]
fn test_serialize_f32_zero() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(0.0);
    assert_eq!(result, Ok(Content::F32(0.0)));
}

#[test]
fn test_serialize_f32_negative() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(-2.71);
    assert_eq!(result, Ok(Content::F32(-2.71)));
}

#[test]
fn test_serialize_f32_large_value() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(1e+38);
    assert_eq!(result, Ok(Content::F32(1e+38)));
}

#[test]
fn test_serialize_f32_small_value() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(1e-38);
    assert_eq!(result, Ok(Content::F32(1e-38)));
}

