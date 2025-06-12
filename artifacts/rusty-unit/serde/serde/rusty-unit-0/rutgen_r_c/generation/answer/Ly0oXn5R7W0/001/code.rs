// Answer 0

#[test]
fn test_visit_unit() {
    struct TestError;

    impl de::Error for TestError {
        fn custom<T>(_: T) -> Self {
            TestError
        }
    }

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Content<'de>;

        fn expecting(&self, _: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }

        fn visit_unit<F>(self) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            Ok(Content::Unit)
        }
    }

    let visitor = TestVisitor;
    let result: Result<Content<'_>, TestError> = visitor.visit_unit();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Content::Unit);
}

