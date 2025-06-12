// Answer 0

#[test]
fn test_visit_i64() {
    struct TestError;
    impl de::Error for TestError {
        fn custom<T>(_: T) -> Self { TestError }
    }

    let visitor = ContentVisitor { value: PhantomData };
    
    let result: Result<Content, TestError> = visitor.visit_i64(42);
    match result {
        Ok(Content::I64(val)) => assert_eq!(val, 42),
        _ => panic!("Expected Ok(Content::I64(42))"),
    }
}

#[test]
fn test_visit_i64_negative() {
    struct TestError;
    impl de::Error for TestError {
        fn custom<T>(_: T) -> Self { TestError }
    }

    let visitor = ContentVisitor { value: PhantomData };

    let result: Result<Content, TestError> = visitor.visit_i64(-10);
    match result {
        Ok(Content::I64(val)) => assert_eq!(val, -10),
        _ => panic!("Expected Ok(Content::I64(-10))"),
    }
}

