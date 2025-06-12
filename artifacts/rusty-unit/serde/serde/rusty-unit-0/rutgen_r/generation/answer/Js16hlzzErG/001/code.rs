// Answer 0

#[test]
fn test_visit_bool_true() {
    struct TestError;

    impl serde::de::Error for TestError {
        fn custom<T>(_: T) -> Self {
            TestError
        }
    }

    struct Visitor;

    impl Visitor {
        fn visit_bool<E>(self, v: bool) -> Result<bool, E>
        where
            E: serde::de::Error,
        {
            Ok(v)
        }
    }

    let visitor = Visitor;
    let result = visitor.visit_bool(true);
    assert_eq!(result, Ok(true));
}

#[test]
fn test_visit_bool_false() {
    struct TestError;

    impl serde::de::Error for TestError {
        fn custom<T>(_: T) -> Self {
            TestError
        }
    }

    struct Visitor;

    impl Visitor {
        fn visit_bool<E>(self, v: bool) -> Result<bool, E>
        where
            E: serde::de::Error,
        {
            Ok(v)
        }
    }

    let visitor = Visitor;
    let result = visitor.visit_bool(false);
    assert_eq!(result, Ok(false));
}

