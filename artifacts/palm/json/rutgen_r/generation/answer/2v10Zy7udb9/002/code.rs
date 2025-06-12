// Answer 0

#[test]
fn test_deserialize_option_with_null() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Option<serde_json::Value>;

        fn visit_none(self) -> Result<Self::Value, serde::de::Error> {
            Ok(None)
        }

        fn visit_some(self, value: serde_json::Value) -> Result<Self::Value, serde::de::Error> {
            Ok(Some(value))
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an option value")
        }
    }

    let value_null = serde_json::Value::Null;
    let visitor = TestVisitor;
    let result = value_null.deserialize_option(visitor).unwrap();
    assert_eq!(result, None);
}

#[test]
fn test_deserialize_option_with_non_null() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Option<serde_json::Value>;

        fn visit_none(self) -> Result<Self::Value, serde::de::Error> {
            Ok(None)
        }

        fn visit_some(self, value: serde_json::Value) -> Result<Self::Value, serde::de::Error> {
            Ok(Some(value))
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an option value")
        }
    }

    let value_some = serde_json::Value::String("Test".to_string());
    let visitor = TestVisitor;
    let result = value_some.deserialize_option(visitor).unwrap();
    assert_eq!(result, Some(value_some));
}

