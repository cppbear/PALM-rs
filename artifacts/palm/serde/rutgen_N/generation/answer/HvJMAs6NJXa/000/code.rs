// Answer 0

#[derive(Debug)]
struct TestError;

impl de::Error for TestError {
    fn custom<T>(msg: T) -> Self where T: std::fmt::Display {
        TestError
    }
}

struct TestVisitor;

impl TestVisitor {
    fn visit_unit<E>(self) -> Result<(), E>
    where
        E: de::Error,
    {
        Ok(())
    }
}

#[test]
fn test_visit_unit_success() {
    let visitor = TestVisitor;
    let result: Result<(), TestError> = visitor.visit_unit();
    assert!(result.is_ok());
}

