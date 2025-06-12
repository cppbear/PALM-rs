// Answer 0

#[derive(Debug)]
struct Content {
    value: i64,
}

impl Content {
    fn I64(value: i64) -> Self {
        Content { value }
    }
}

#[derive(Debug)]
struct Serializer;

impl Serializer {
    fn serialize_i64(self, v: i64) -> Result<Content, String> {
        Ok(Content::I64(v))
    }
}

#[test]
fn test_serialize_i64_positive() {
    let serializer = Serializer;
    let result = serializer.serialize_i64(42);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, 42);
}

#[test]
fn test_serialize_i64_negative() {
    let serializer = Serializer;
    let result = serializer.serialize_i64(-1);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, -1);
}

#[test]
fn test_serialize_i64_zero() {
    let serializer = Serializer;
    let result = serializer.serialize_i64(0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, 0);
}

