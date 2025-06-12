// Answer 0

#[derive(Debug)]
struct Content {
    value: String,
}

impl Content {
    fn String(value: String) -> Content {
        Content { value }
    }
}

struct SerdeSerializer;

impl SerdeSerializer {
    fn serialize_str(self, value: &str) -> Result<Content, &'static str> {
        Ok(Content::String(value.to_owned()))
    }
}

#[test]
fn test_serialize_str() {
    let serializer = SerdeSerializer;
    let result = serializer.serialize_str("test string").unwrap();
    assert_eq!(result.value, "test string");
}

#[test]
fn test_serialize_empty_str() {
    let serializer = SerdeSerializer;
    let result = serializer.serialize_str("").unwrap();
    assert_eq!(result.value, "");
}

