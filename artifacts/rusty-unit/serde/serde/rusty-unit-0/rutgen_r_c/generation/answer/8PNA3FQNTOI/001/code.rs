// Answer 0

#[test]
fn test_visit_i64() {
    struct TestError;

    impl de::Error for TestError {
        fn custom<T>(msg: T) -> Self
        where
            T: std::fmt::Display,
        {
            TestError
        }
    }

    let visitor = ContentVisitor { value: PhantomData };
    let value: i64 = 42;

    let result: Result<Content, TestError> = visitor.visit_i64(value);
    assert_eq!(result, Ok(Content::I64(42)));
}

#[test]
fn test_visit_i64_negative() {
    struct TestError;

    impl de::Error for TestError {
        fn custom<T>(msg: T) -> Self
        where
            T: std::fmt::Display,
        {
            TestError
        }
    }

    let visitor = ContentVisitor { value: PhantomData };
    let value: i64 = -42;

    let result: Result<Content, TestError> = visitor.visit_i64(value);
    assert_eq!(result, Ok(Content::I64(-42)));
}

#[test]
fn test_visit_i64_zero() {
    struct TestError;

    impl de::Error for TestError {
        fn custom<T>(msg: T) -> Self
        where
            T: std::fmt::Display,
        {
            TestError
        }
    }

    let visitor = ContentVisitor { value: PhantomData };
    let value: i64 = 0;

    let result: Result<Content, TestError> = visitor.visit_i64(value);
    assert_eq!(result, Ok(Content::I64(0)));
}

