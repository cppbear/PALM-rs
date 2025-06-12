// Answer 0

#[derive(Debug)]
struct Content {
    value: i64,
}

impl Content {
    fn I64(value: i64) -> Self {
        Content { value }
    }
}

mod de {
    pub trait Error {}
    pub struct MockError;
    
    impl Error for MockError {}
}

fn visit_i64<F>(value: i64) -> Result<Content, F>
where
    F: de::Error,
{
    Ok(Content::I64(value))
}

#[test]
fn test_visit_i64_positive() {
    let result: Result<Content, de::MockError> = visit_i64(42);
    assert!(result.is_ok());
    if let Ok(content) = result {
        assert_eq!(content.value, 42);
    }
}

#[test]
fn test_visit_i64_negative() {
    let result: Result<Content, de::MockError> = visit_i64(-42);
    assert!(result.is_ok());
    if let Ok(content) = result {
        assert_eq!(content.value, -42);
    }
}

#[test]
fn test_visit_i64_zero() {
    let result: Result<Content, de::MockError> = visit_i64(0);
    assert!(result.is_ok());
    if let Ok(content) = result {
        assert_eq!(content.value, 0);
    }
}

