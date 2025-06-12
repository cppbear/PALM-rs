// Answer 0

#[test]
fn test_deserialize_bool_valid() {
    struct TestVisitor {
        value: Option<bool>,
    }

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value)
        }
        
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean")
        }
    }

    let value = serde_json::Value::Bool(true);
    let visitor = TestVisitor { value: None };
    let result = value.deserialize_bool(visitor);
    assert_eq!(result, Ok(true));
}

#[test]
fn test_deserialize_bool_invalid() {
    struct TestVisitor {
        value: Option<bool>,
    }

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value)
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean")
        }
    }

    let value = serde_json::Value::String("not_a_bool".to_string());
    let visitor = TestVisitor { value: None };
    let result = value.deserialize_bool(visitor);
    assert!(result.is_err());
}

