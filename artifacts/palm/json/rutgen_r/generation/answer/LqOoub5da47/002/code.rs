// Answer 0

#[test]
fn test_deserialize_option_with_null() {
    use serde::de::{self, Visitor};
    use serde_json::Value;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<Value>;

        fn visit_none(self) -> Result<Self::Value, de::Error> {
            Ok(None)
        }

        fn visit_some<V>(self, value: V) -> Result<Self::Value, de::Error>
        where
            V: serde::de::Deserialize<'de>,
        {
            Ok(Some(value))
        }
    }

    let value = Value::Null;
    let visitor = TestVisitor;

    let result: Result<Option<Value>, serde::de::Error> = value.deserialize_option(visitor);
    assert_eq!(result, Ok(None));
}

#[test]
fn test_deserialize_option_with_non_null() {
    use serde::de::{self, Visitor};
    use serde_json::Value;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<Value>;

        fn visit_none(self) -> Result<Self::Value, de::Error> {
            Ok(None)
        }

        fn visit_some<V>(self, value: V) -> Result<Self::Value, de::Error>
        where
            V: serde::de::Deserialize<'de>,
        {
            Ok(Some(value))
        }
    }

    let value = Value::String("Test".to_string());
    let visitor = TestVisitor;

    let result: Result<Option<Value>, serde::de::Error> = value.deserialize_option(visitor);
    assert_eq!(result, Ok(Some(Value::String("Test".to_string()))));
}

