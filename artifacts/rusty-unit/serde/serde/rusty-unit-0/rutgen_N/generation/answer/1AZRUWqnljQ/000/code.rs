// Answer 0

#[derive(Debug)]
struct TagOrContentField {
    tag: String,
    content: String,
}

impl TagOrContentField {
    pub fn new(tag: &str, content: &str) -> Self {
        Self {
            tag: tag.to_string(),
            content: content.to_string(),
        }
    }
}

mod de {
    pub trait Error {
        fn invalid_value<T>(_unexpected: T, _expected: &str) -> Self;
    }
}

#[derive(Debug)]
struct TestError;

impl de::Error for TestError {
    fn invalid_value<T>(_unexpected: T, _expected: &str) -> Self {
        TestError
    }
}

impl TagOrContentField {
    fn visit_bytes<E>(&self, field: &[u8]) -> Result<&str, E>
    where
        E: de::Error,
    {
        if field == self.tag.as_bytes() {
            Ok("Tag")
        } else if field == self.content.as_bytes() {
            Ok("Content")
        } else {
            Err(de::Error::invalid_value(field, "expected tag or content"))
        }
    }
}

#[test]
fn test_visit_bytes_with_tag() {
    let field = TagOrContentField::new("tag", "content");
    let result = field.visit_bytes(b"tag");
    assert_eq!(result, Ok("Tag"));
}

#[test]
fn test_visit_bytes_with_content() {
    let field = TagOrContentField::new("tag", "content");
    let result = field.visit_bytes(b"content");
    assert_eq!(result, Ok("Content"));
}

#[test]
fn test_visit_bytes_with_invalid_value() {
    let field = TagOrContentField::new("tag", "content");
    let result: Result<&str, TestError> = field.visit_bytes(b"invalid");
    assert!(result.is_err());
}

