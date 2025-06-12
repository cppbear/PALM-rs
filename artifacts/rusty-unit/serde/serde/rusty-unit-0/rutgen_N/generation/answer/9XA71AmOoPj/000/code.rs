// Answer 0

#[derive(Debug)]
struct Content {
    inner: Option<()>,
}

impl Content {
    fn None() -> Self {
        Content { inner: None }
    }
}

#[derive(Debug)]
struct Serializer;

impl Serializer {
    fn serialize_none(self) -> Result<Content, &'static str> {
        Ok(Content::None())
    }
}

#[test]
fn test_serialize_none() {
    let serializer = Serializer;
    let result = serializer.serialize_none();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().inner, None);
}

