// Answer 0

#[test]
fn test_deserialize_any_success() {
    struct TestVisitor {
        value: Option<serde_json::Value>,
    }

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = serde_json::Value;

        fn visit_map<M>(self, _map: M) -> Result<Self::Value, M::Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            Ok(serde_json::json!({"key": "value"}))
        }
    }

    let input = serde_json::from_str(r#"{"key": "value"}"#).unwrap();
    let result: Result<serde_json::Value, _> = input.deserialize_any(TestVisitor { value: None });

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), serde_json::json!({"key": "value"}));
}

#[test]
#[should_panic]
fn test_deserialize_any_error() {
    struct TestVisitor {
        value: Option<serde_json::Value>,
    }

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = serde_json::Value;

        fn visit_map<M>(self, _map: M) -> Result<Self::Value, M::Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            Ok(serde_json::json!({"key": "value"}))
        }
    }

    let input = serde_json::from_str(r#"{"key": "value", "extra_key": "extra_value"}"#).unwrap();
    let result: Result<serde_json::Value, _> = input.deserialize_any(TestVisitor { value: None });

    if result.is_err() {
        panic!("Expected to succeed, but failed with error: {:?}", result.err());
    }
}

