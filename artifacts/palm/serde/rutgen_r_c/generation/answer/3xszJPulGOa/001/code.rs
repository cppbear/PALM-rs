// Answer 0

#[test]
fn test_visit_none() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<()>;

        fn expecting(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, _deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            panic!("Unexpected call to visit_some");
        }
    }

    let visitor = TestVisitor;
    let result: Result<Option<()>, _> = visitor.visit_none();
    assert_eq!(result, Ok(None));
}

