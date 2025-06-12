// Answer 0

#[derive(Debug)]
struct Content {
    data: Vec<u8>,
}

impl Content {
    fn ByteBuf(value: Vec<u8>) -> Self {
        Content { data: value }
    }
}

mod de {
    pub trait Error {}
}

#[derive(Debug)]
struct TestError;

impl de::Error for TestError {}

struct TestVisitor;

impl TestVisitor {
    fn visit_byte_buf<F>(self, value: Vec<u8>) -> Result<Content, F>
    where
        F: de::Error,
    {
        Ok(Content::ByteBuf(value))
    }
}

#[test]
fn test_visit_byte_buf_empty() {
    let visitor = TestVisitor;
    let value = Vec::new();
    let result: Result<Content, TestError> = visitor.visit_byte_buf(value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().data, vec![]);
}

#[test]
fn test_visit_byte_buf_non_empty() {
    let visitor = TestVisitor;
    let value = vec![1, 2, 3, 4, 5];
    let result: Result<Content, TestError> = visitor.visit_byte_buf(value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().data, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_visit_byte_buf_large() {
    let visitor = TestVisitor;
    let value = (0..1000).map(|x| x as u8).collect::<Vec<u8>>();
    let result: Result<Content, TestError> = visitor.visit_byte_buf(value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().data, (0..1000).collect::<Vec<u8>>());
}

