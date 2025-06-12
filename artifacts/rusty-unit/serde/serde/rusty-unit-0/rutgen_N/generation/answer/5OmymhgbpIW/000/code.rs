// Answer 0

#[derive(Debug)]
struct Content<'de> {
    value: &'de str,
}

impl<'de> Content<'de> {
    fn str(value: &'de str) -> Self {
        Content { value }
    }
}

mod de {
    pub trait Error {}
}

struct TestError;

impl de::Error for TestError {}

fn visit_borrowed_str<F>(value: &'static str) -> Result<Content<'static>, F>
where
    F: de::Error,
{
    Ok(Content::str(value))
}

#[test]
fn test_visit_borrowed_str() {
    let result: Result<Content, TestError> = visit_borrowed_str::<TestError>("test string");
    assert!(result.is_ok());
    if let Ok(content) = result {
        assert_eq!(content.value, "test string");
    }
}

#[test]
#[should_panic]
fn test_visit_borrowed_str_empty() {
    let _: Result<Content, TestError> = visit_borrowed_str::<TestError>("");
}

