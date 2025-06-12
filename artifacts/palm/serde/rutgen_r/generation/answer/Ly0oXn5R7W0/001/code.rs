// Answer 0

#[test]
fn test_visit_unit() {
    struct TestError;

    impl serde::de::Error for TestError {
        fn custom<T>(msg: T) -> Self where T: std::fmt::Display {
            TestError
        }
    }

    struct TestVisitor;

    impl TestVisitor {
        fn visit_unit<F>(self) -> Result<(), F>
        where
            F: serde::de::Error,
        {
            Ok(())
        }
    }

    let visitor = TestVisitor;
    let result: Result<(), TestError> = visitor.visit_unit();

    assert!(result.is_ok());
}

