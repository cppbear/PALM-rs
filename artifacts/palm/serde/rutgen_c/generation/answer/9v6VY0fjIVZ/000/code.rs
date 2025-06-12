// Answer 0

#[test]
fn test_visit_f64() {
    struct TestError;

    impl de::Error for TestError {
        fn custom<T>(msg: T) -> Self where T: std::fmt::Display {
            TestError
        }
    }

    let visitor = ContentVisitor { value: std::marker::PhantomData };

    let result: Result<Content, TestError> = visitor.visit_f64(3.14);
    assert_eq!(result, Ok(Content::F64(3.14)));
}

#[test]
fn test_visit_f64_negative() {
    struct TestError;

    impl de::Error for TestError {
        fn custom<T>(msg: T) -> Self where T: std::fmt::Display {
            TestError
        }
    }

    let visitor = ContentVisitor { value: std::marker::PhantomData };

    let result: Result<Content, TestError> = visitor.visit_f64(-2.71);
    assert_eq!(result, Ok(Content::F64(-2.71)));
}

