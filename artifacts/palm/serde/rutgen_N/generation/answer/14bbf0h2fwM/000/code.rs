// Answer 0

#[test]
fn test_visit_u16_success() {
    struct TestError;
    impl de::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    struct MockVisitor;

    impl MockVisitor {
        fn new() -> Self {
            MockVisitor
        }

        fn visit_u16(self, value: u16) -> Result<u16, TestError> {
            Ok(value)
        }
    }

    struct ContentVisitor;

    impl ContentVisitor {
        fn new() -> Self {
            ContentVisitor
        }

        fn visit_u16(self, value: u16) -> Result<u16, TestError> {
            MockVisitor::new().visit_u16(value)
        }
    }

    enum TagOrContent {
        Content(u16),
    }

    let result = ContentVisitor::new().visit_u16(42).map(TagOrContent::Content);
    assert_eq!(result, Ok(TagOrContent::Content(42)));
}

#[test]
#[should_panic]
fn test_visit_u16_failure() {
    struct TestError;
    impl de::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    struct MockVisitor;

    impl MockVisitor {
        fn new() -> Self {
            MockVisitor
        }

        fn visit_u16(self, _value: u16) -> Result<u16, TestError> {
            Err(TestError)
        }
    }

    struct ContentVisitor;

    impl ContentVisitor {
        fn new() -> Self {
            ContentVisitor
        }

        fn visit_u16(self, value: u16) -> Result<u16, TestError> {
            MockVisitor::new().visit_u16(value)
        }
    }

    let _result = ContentVisitor::new().visit_u16(42).map(TagOrContent::Content).unwrap(); // This will panic on Err
}

