// Answer 0

#[test]
fn test_visit_unit() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = IgnoredAny;

        fn expecting(&self, _formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            Ok(())
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Ok(IgnoredAny)
        }
    }

    let visitor = TestVisitor;
    let result: Result<IgnoredAny, ()> = visitor.visit_unit();
    assert_eq!(result, Ok(IgnoredAny));
}

