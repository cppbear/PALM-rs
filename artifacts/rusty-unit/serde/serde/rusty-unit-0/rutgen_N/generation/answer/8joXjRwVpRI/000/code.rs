// Answer 0

#[derive(Debug)]
struct Content {
    u8_value: u8,
}

impl Content {
    fn U8(value: u8) -> Self {
        Content { u8_value: value }
    }
}

mod de {
    pub trait Error {}
}

struct TestError;

impl de::Error for TestError {}

struct TestVisitor;

impl TestVisitor {
    fn visit_u8<F>(self, value: u8) -> Result<Content, F>
    where
        F: de::Error,
    {
        Ok(Content::U8(value))
    }
}

#[test]
fn test_visit_u8() {
    let visitor = TestVisitor;
    let value = 255;
    let result: Result<Content, TestError> = visitor.visit_u8(value);
    assert!(result.is_ok());
    
    if let Ok(content) = result {
        assert_eq!(content.u8_value, value);
    }
}

#[test]
fn test_visit_u8_zero() {
    let visitor = TestVisitor;
    let value = 0;
    let result: Result<Content, TestError> = visitor.visit_u8(value);
    assert!(result.is_ok());
    
    if let Ok(content) = result {
        assert_eq!(content.u8_value, value);
    }
}

