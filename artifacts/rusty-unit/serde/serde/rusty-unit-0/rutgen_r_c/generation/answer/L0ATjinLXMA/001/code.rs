// Answer 0

#[test]
fn test_visit_i8_valid() {
    struct TestError;
    impl de::Error for TestError {}

    let visitor = ContentVisitor { value: PhantomData };
    let result: Result<Content, TestError> = visitor.visit_i8(42);
    assert_eq!(result, Ok(Content::I8(42)));
}

#[test]
fn test_visit_i8_negative() {
    struct TestError;
    impl de::Error for TestError {}

    let visitor = ContentVisitor { value: PhantomData };
    let result: Result<Content, TestError> = visitor.visit_i8(-15);
    assert_eq!(result, Ok(Content::I8(-15)));
}

#[test]
fn test_visit_i8_zero() {
    struct TestError;
    impl de::Error for TestError {}

    let visitor = ContentVisitor { value: PhantomData };
    let result: Result<Content, TestError> = visitor.visit_i8(0);
    assert_eq!(result, Ok(Content::I8(0)));
}

