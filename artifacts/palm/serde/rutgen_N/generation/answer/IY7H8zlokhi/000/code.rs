// Answer 0

#[derive(Debug)]
struct Content {
    value: i16,
}

impl Content {
    fn I16(value: i16) -> Self {
        Content { value }
    }
}

trait Error {}

#[derive(Debug)]
struct TestError;

impl Error for TestError {}

struct Visitor;

impl Visitor {
    fn visit_i16<F>(self, value: i16) -> Result<Content, F>
    where
        F: Error,
    {
        Ok(Content::I16(value))
    }
}

#[test]
fn test_visit_i16_positive_value() {
    let visitor = Visitor;
    let result = visitor.visit_i16::<TestError>(42);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, 42);
}

#[test]
fn test_visit_i16_negative_value() {
    let visitor = Visitor;
    let result = visitor.visit_i16::<TestError>(-27);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, -27);
}

#[test]
fn test_visit_i16_zero_value() {
    let visitor = Visitor;
    let result = visitor.visit_i16::<TestError>(0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, 0);
}

