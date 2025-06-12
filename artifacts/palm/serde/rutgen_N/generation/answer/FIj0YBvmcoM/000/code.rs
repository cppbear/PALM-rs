// Answer 0

#[test]
fn test_visit_borrowed_bytes() {
    struct TestError;
    impl serde::de::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    struct TestVisitor;

    impl TestVisitor {
        fn visit_borrowed_bytes<'a, E>(self, v: &'a [u8]) -> Result<&'a [u8], E>
        where
            E: serde::de::Error,
        {
            Ok(v)
        }
    }

    let visitor = TestVisitor;
    let bytes: &[u8] = b"test";
    let result: Result<&[u8], TestError> = visitor.visit_borrowed_bytes(bytes);
    assert_eq!(result.unwrap(), bytes);
}

#[test]
fn test_visit_borrowed_bytes_empty() {
    struct TestError;
    impl serde::de::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    struct TestVisitor;

    impl TestVisitor {
        fn visit_borrowed_bytes<'a, E>(self, v: &'a [u8]) -> Result<&'a [u8], E>
        where
            E: serde::de::Error,
        {
            Ok(v)
        }
    }

    let visitor = TestVisitor;
    let bytes: &[u8] = b"";
    let result: Result<&[u8], TestError> = visitor.visit_borrowed_bytes(bytes);
    assert_eq!(result.unwrap(), bytes);
}

