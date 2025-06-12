// Answer 0

#[derive(Debug)]
struct Content {
    value: f32,
}

impl Content {
    fn F32(v: f32) -> Content {
        Content { value: v }
    }
}

#[derive(Debug)]
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
    assert_eq!(result.unwrap().value, 3.14);
}

#[test]
fn test_serialize_f32_negative() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(-2.71);
    assert_eq!(result.unwrap().value, -2.71);
}

#[test]
fn test_serialize_f32_zero() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(0.0);
    assert_eq!(result.unwrap().value, 0.0);
}

#[test]
fn test_serialize_f32_boundary() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(f32::MAX);
    assert_eq!(result.unwrap().value, f32::MAX);
    
    let result = serializer.serialize_f32(f32::MIN);
    assert_eq!(result.unwrap().value, f32::MIN);
}

