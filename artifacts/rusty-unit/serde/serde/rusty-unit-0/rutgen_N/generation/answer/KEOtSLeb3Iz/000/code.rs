// Answer 0

#[derive(Debug)]
struct Content {
    value: u8,
}

impl Content {
    fn U8(v: u8) -> Self {
        Content { value: v }
    }
}

#[derive(Debug)]
struct Serializer;

impl Serializer {
    fn serialize_u8(self, v: u8) -> Result<Content, &'static str> {
        Ok(Content::U8(v))
    }
}

#[test]
fn test_serialize_u8_valid() {
    let serializer = Serializer;
    let result = serializer.serialize_u8(42);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, 42);
}

#[test]
fn test_serialize_u8_boundary() {
    let serializer = Serializer;
    let result_zero = serializer.serialize_u8(0);
    assert!(result_zero.is_ok());
    assert_eq!(result_zero.unwrap().value, 0);

    let result_max = serializer.serialize_u8(255);
    assert!(result_max.is_ok());
    assert_eq!(result_max.unwrap().value, 255);
}

