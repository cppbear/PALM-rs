// Answer 0

#[derive(Debug)]
struct DummyError;

impl serde::de::Error for DummyError {
    fn custom<T>(_msg: T) -> Self {
        DummyError
    }
}

struct DummyVisitor;

impl DummyVisitor {
    fn visit_i8<F>(self, value: i8) -> Result<Content, F>
    where
        F: de::Error,
    {
        Ok(Content::I8(value))
    }
}

#[derive(Debug)]
enum Content {
    I8(i8),
}

#[test]
fn test_visit_i8() {
    let visitor = DummyVisitor;
    let result: Result<Content, DummyError> = visitor.visit_i8(42);
    assert!(result.is_ok());
    if let Ok(content) = result {
        match content {
            Content::I8(value) => assert_eq!(value, 42),
            _ => panic!("Expected Content::I8"),
        }
    }
}

#[test]
fn test_visit_i8_negative_value() {
    let visitor = DummyVisitor;
    let result: Result<Content, DummyError> = visitor.visit_i8(-1);
    assert!(result.is_ok());
    if let Ok(content) = result {
        match content {
            Content::I8(value) => assert_eq!(value, -1),
            _ => panic!("Expected Content::I8"),
        }
    }
}

