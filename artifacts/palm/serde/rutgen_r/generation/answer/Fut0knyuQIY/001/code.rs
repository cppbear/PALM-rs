// Answer 0

#[test]
fn test_visit_bool_true() {
    struct MockError;
    impl de::Error for MockError {}

    struct Visitor;
    impl Visitor {
        fn visit_bool<F>(self, value: bool) -> Result<Content, F>
        where
            F: de::Error,
        {
            Ok(Content::Bool(value))
        }
    }

    let visitor = Visitor;
    let result = visitor.visit_bool::<MockError>(true);
    assert_eq!(result, Ok(Content::Bool(true)));
}

#[test]
fn test_visit_bool_false() {
    struct MockError;
    impl de::Error for MockError {}

    struct Visitor;
    impl Visitor {
        fn visit_bool<F>(self, value: bool) -> Result<Content, F>
        where
            F: de::Error,
        {
            Ok(Content::Bool(value))
        }
    }

    let visitor = Visitor;
    let result = visitor.visit_bool::<MockError>(false);
    assert_eq!(result, Ok(Content::Bool(false)));
}

