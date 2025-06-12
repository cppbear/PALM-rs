// Answer 0

#[derive(Debug)]
struct TestError;

impl de::Error for TestError {
    fn custom<T>(_msg: T) -> Self {
        TestError
    }
}

#[derive(Debug)]
struct Content<'a> {
    bytes: &'a [u8],
}

impl<'de> Content<'de> {
    fn bytes(value: &'de [u8]) -> Self {
        Content { bytes: value }
    }
}

#[test]
fn test_visit_borrowed_bytes_success() {
    let value: &[u8] = b"test";
    let result: Result<Content, TestError> = visit_borrowed_bytes(value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().bytes, b"test");
}

#[should_panic]
#[test]
fn test_visit_borrowed_bytes_edge_case() {
    let value: &[u8] = &[];
    let result: Result<Content, TestError> = visit_borrowed_bytes(value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().bytes, &[]);
}

