// Answer 0

#[test]
fn test_visit_string() {
    struct TestError;
    impl de::Error for TestError {
        fn custom<T: std::fmt::Display>(_: T) -> Self {
            TestError
        }
    }

    let visitor = ContentVisitor { value: PhantomData };
    let result: Result<Content, TestError> = visitor.visit_string("test".to_owned());

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Content::String("test".to_owned()));
}

#[test]
fn test_visit_string_empty() {
    struct TestError;
    impl de::Error for TestError {
        fn custom<T: std::fmt::Display>(_: T) -> Self {
            TestError
        }
    }

    let visitor = ContentVisitor { value: PhantomData };
    let result: Result<Content, TestError> = visitor.visit_string("".to_owned());

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Content::String("".to_owned()));
}

