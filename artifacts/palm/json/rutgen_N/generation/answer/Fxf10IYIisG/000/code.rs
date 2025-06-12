// Answer 0

#[test]
fn test_deserialize_ignored_any() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
            Ok(())
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("unit")
        }
    }

    let visitor = TestVisitor;
    let result: Result<(), serde::de::Error> = deserialize_ignored_any(visitor);
    assert!(result.is_ok());
}

