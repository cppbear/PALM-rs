// Answer 0

#[test]
fn test_deserialize_option_null() {
    use serde_json::Value;
    use serde::de::{Deserialize, Deserializer, Visitor, Error as DeError};

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<Value>;

        fn visit_none(self) -> Result<Self::Value, DeError> {
            Ok(None)
        }

        fn visit_some(self, value: Value) -> Result<Self::Value, DeError> {
            Ok(Some(value))
        }
    }

    let value = Value::Null;
    let visitor = TestVisitor;

    let result = value.deserialize_option(visitor).unwrap();
    assert_eq!(result, None);
}

