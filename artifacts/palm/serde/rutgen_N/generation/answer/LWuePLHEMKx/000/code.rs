// Answer 0

#[derive(Debug)]
struct MockError;

impl de::Error for MockError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        MockError
    }
}

struct MockVisitor;

impl MockVisitor {
    fn visit_string(self, value: String) -> Result<Content, MockError> {
        Ok(Content::String(value))
    }
}

#[test]
fn test_visit_string_success() {
    let visitor = MockVisitor;
    let result: Result<Content, MockError> = visitor.visit_string("test".to_string());
    assert!(result.is_ok());
    if let Ok(content) = result {
        match content {
            Content::String(s) => assert_eq!(s, "test"),
            _ => panic!("Expected Content::String"),
        }
    }
}

#[test]
fn test_visit_string_empty() {
    let visitor = MockVisitor;
    let result: Result<Content, MockError> = visitor.visit_string("".to_string());
    assert!(result.is_ok());
    if let Ok(content) = result {
        match content {
            Content::String(s) => assert_eq!(s, ""),
            _ => panic!("Expected Content::String"),
        }
    }
}

