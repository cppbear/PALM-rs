// Answer 0

#[derive(Debug)]
struct Content {
    value: u8,
}

impl Content {
    fn U8(value: u8) -> Self {
        Content { value }
    }
}

mod de {
    pub trait Error {}
}

#[derive(Debug)]
struct TestError;

impl de::Error for TestError {}

struct Visitor;

impl Visitor {
    fn visit_u8<F>(self, value: u8) -> Result<Content, F>
    where
        F: de::Error,
    {
        Ok(Content::U8(value))
    }
}

#[test]
fn test_visit_u8_with_zero() {
    let visitor = Visitor;
    let result: Result<Content, TestError> = visitor.visit_u8(0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, 0);
}

#[test]
fn test_visit_u8_with_max_value() {
    let visitor = Visitor;
    let result: Result<Content, TestError> = visitor.visit_u8(255);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, 255);
}

#[test]
fn test_visit_u8_with_mid_value() {
    let visitor = Visitor;
    let result: Result<Content, TestError> = visitor.visit_u8(128);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, 128);
}

