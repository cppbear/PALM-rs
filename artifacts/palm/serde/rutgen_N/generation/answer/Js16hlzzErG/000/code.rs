// Answer 0

#[derive(Debug)]
struct DummyError;

impl serde::de::Error for DummyError {
    fn custom<T>(_msg: T) -> Self {
        DummyError
    }
}

#[test]
fn test_visit_bool() {
    struct TestVisitor;

    impl serde::de::Visitor for TestVisitor {
        type Value = bool;

        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v)
        }
    }

    let visitor = TestVisitor;
    let result: Result<bool, DummyError> = visitor.visit_bool(true);
    assert_eq!(result, Ok(true));

    let result: Result<bool, DummyError> = visitor.visit_bool(false);
    assert_eq!(result, Ok(false));
}

