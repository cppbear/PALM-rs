// Answer 0

#[test]
fn test_deserialize_bool_success() {
    struct TestVisitor {
        value: Option<bool>,
    }

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean")
        }
    }

    let value = serde_json::Value::Bool(true);
    let visitor = TestVisitor { value: None };
    let result = value.deserialize_bool(visitor).unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_deserialize_bool_fail_not_a_bool() {
    struct TestVisitor {
        value: Option<bool>,
    }

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean")
        }
    }

    let value = serde_json::Value::String("not a bool".to_string());
    let visitor = TestVisitor { value: None };
    let result = value.deserialize_bool(visitor);
    assert!(result.is_err());
}

