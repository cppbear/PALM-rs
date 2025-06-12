// Answer 0

#[derive(Debug)]
struct MockError;

impl de::Error for MockError {
    fn custom<T>(_msg: T) -> Self {
        MockError
    }
}

struct MockVisitor;

impl MockVisitor {
    fn visit_i64<F>(self, value: i64) -> Result<Content, F>
    where
        F: de::Error,
    {
        Ok(Content::I64(value))
    }
}

#[derive(Debug)]
enum Content {
    I64(i64),
}

#[test]
fn test_visit_i64() {
    let visitor = MockVisitor;
    let value = 42i64;
    let result: Result<Content, MockError> = visitor.visit_i64(value);

    assert!(result.is_ok());
    if let Ok(content) = result {
        match content {
            Content::I64(v) => assert_eq!(v, value),
            _ => panic!("Unexpected content type"),
        }
    }
}

#[test]
fn test_visit_i64_negative() {
    let visitor = MockVisitor;
    let value = -42i64;
    let result: Result<Content, MockError> = visitor.visit_i64(value);

    assert!(result.is_ok());
    if let Ok(content) = result {
        match content {
            Content::I64(v) => assert_eq!(v, value),
            _ => panic!("Unexpected content type"),
        }
    }
}

