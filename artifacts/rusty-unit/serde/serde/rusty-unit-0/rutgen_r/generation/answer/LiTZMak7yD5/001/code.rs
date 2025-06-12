// Answer 0

#[derive(Debug)]
struct Content {
    value: bool,
}

impl Content {
    fn Bool(v: bool) -> Self {
        Content { value: v }
    }
}

struct Serializer;

impl Serializer {
    fn serialize_bool(self, v: bool) -> Result<Content, &'static str> {
        Ok(Content::Bool(v))
    }
}

#[test]
fn test_serialize_bool_true() {
    let serializer = Serializer;
    let result = serializer.serialize_bool(true);
    assert_eq!(result, Ok(Content::Bool(true)));
}

#[test]
fn test_serialize_bool_false() {
    let serializer = Serializer;
    let result = serializer.serialize_bool(false);
    assert_eq!(result, Ok(Content::Bool(false)));
}

