// Answer 0

#[derive(Debug)]
struct Content {
    value: u64,
}

impl Content {
    fn U64(v: u64) -> Self {
        Content { value: v }
    }
}

#[derive(Debug)]
struct Serializer;

impl Serializer {
    fn serialize_u64(self, v: u64) -> Result<Content, &'static str> {
        Ok(Content::U64(v))
    }
}

#[test]
fn test_serialize_u64_zero() {
    let serializer = Serializer;
    let result = serializer.serialize_u64(0);
    assert_eq!(result, Ok(Content::U64(0)));
}

#[test]
fn test_serialize_u64_max() {
    let serializer = Serializer;
    let result = serializer.serialize_u64(u64::MAX);
    assert_eq!(result, Ok(Content::U64(u64::MAX)));
}

#[test]
fn test_serialize_u64_small_positive() {
    let serializer = Serializer;
    let result = serializer.serialize_u64(42);
    assert_eq!(result, Ok(Content::U64(42)));
}

#[test]
fn test_serialize_u64_large_value() {
    let serializer = Serializer;
    let result = serializer.serialize_u64(1_000_000_000_000);
    assert_eq!(result, Ok(Content::U64(1_000_000_000_000)));
}

