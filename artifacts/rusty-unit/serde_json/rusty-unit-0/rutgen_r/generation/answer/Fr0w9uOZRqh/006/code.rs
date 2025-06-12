// Answer 0

#[test]
fn test_deserialize_any_null() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, Error> {
            Ok(())
        }

        fn visit_bool(self, _: bool) -> Result<Self::Value, Error> {
            unreachable!()
        }

        fn visit_string(self, _: &str) -> Result<Self::Value, Error> {
            unreachable!()
        }

        fn visit_array<V>(self, _: V) -> Result<Self::Value, Error>
        where
            V: Iterator<Item = Result<Self::Value, Error>>,
        {
            unreachable!()
        }

        fn visit_object<V>(self, _: V) -> Result<Self::Value, Error>
        where
            V: Iterator<Item = Result<Self::Value, Error>>,
        {
            unreachable!()
        }
    }

    let value = Value::Null;
    let visitor = TestVisitor;

    let result = value.deserialize_any(visitor);
    assert!(result.is_ok());
}

