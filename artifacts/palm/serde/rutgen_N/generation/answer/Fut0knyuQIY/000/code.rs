// Answer 0

#[derive(Debug)]
struct Content {
    value: bool,
}

impl Content {
    fn Bool(value: bool) -> Self {
        Content { value }
    }
}

mod de {
    pub trait Error {}
}

struct TestError;

impl de::Error for TestError {}

struct Visitor;

impl Visitor {
    fn visit_bool<F>(self, value: bool) -> Result<Content, F>
    where
        F: de::Error,
    {
        Ok(Content::Bool(value))
    }
}

#[test]
fn test_visit_bool_true() {
    let visitor = Visitor;
    let result: Result<Content, TestError> = visitor.visit_bool(true);
    assert!(result.is_ok());
    if let Ok(content) = result {
        assert_eq!(content.value, true);
    }
}

#[test]
fn test_visit_bool_false() {
    let visitor = Visitor;
    let result: Result<Content, TestError> = visitor.visit_bool(false);
    assert!(result.is_ok());
    if let Ok(content) = result {
        assert_eq!(content.value, false);
    }
}

