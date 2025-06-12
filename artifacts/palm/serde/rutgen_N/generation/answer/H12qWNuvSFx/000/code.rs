// Answer 0

#[derive(Debug)]
struct Content {
    value: f64,
}

impl Content {
    fn F64(value: f64) -> Self {
        Content { value }
    }
}

struct Serializer;

impl Serializer {
    fn serialize_f64(self, v: f64) -> Result<Content, &'static str> {
        Ok(Content::F64(v))
    }
}

#[test]
fn test_serialize_f64_positive() {
    let serializer = Serializer;
    let result = serializer.serialize_f64(42.0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, 42.0);
}

#[test]
fn test_serialize_f64_negative() {
    let serializer = Serializer;
    let result = serializer.serialize_f64(-3.14);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, -3.14);
}

#[test]
fn test_serialize_f64_zero() {
    let serializer = Serializer;
    let result = serializer.serialize_f64(0.0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, 0.0);
}

