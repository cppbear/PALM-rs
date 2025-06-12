// Answer 0

#[derive(Debug)]
struct Content {
    value: i64,
}

impl Content {
    fn I64(v: i64) -> Self {
        Content { value: v }
    }
}

struct Serializer;

impl Serializer {
    fn serialize_i64(self, v: i64) -> Result<Content, ()> {
        Ok(Content::I64(v))
    }
}

#[test]
fn test_serialize_i64_positive() {
    let serializer = Serializer;
    let result = serializer.serialize_i64(42);
    assert_eq!(result.unwrap(), Content::I64(42));
}

#[test]
fn test_serialize_i64_negative() {
    let serializer = Serializer;
    let result = serializer.serialize_i64(-42);
    assert_eq!(result.unwrap(), Content::I64(-42));
}

#[test]
fn test_serialize_i64_zero() {
    let serializer = Serializer;
    let result = serializer.serialize_i64(0);
    assert_eq!(result.unwrap(), Content::I64(0));
}

#[test]
fn test_serialize_i64_large() {
    let serializer = Serializer;
    let result = serializer.serialize_i64(i64::MAX);
    assert_eq!(result.unwrap(), Content::I64(i64::MAX));
}

#[test]
fn test_serialize_i64_small() {
    let serializer = Serializer;
    let result = serializer.serialize_i64(i64::MIN);
    assert_eq!(result.unwrap(), Content::I64(i64::MIN));
}

