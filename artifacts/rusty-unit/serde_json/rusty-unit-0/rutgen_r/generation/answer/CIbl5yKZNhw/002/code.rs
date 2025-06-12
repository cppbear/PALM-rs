// Answer 0

fn test_deserialize_enum_valid_case() {
    struct VisitorMock;
    impl<'de> serde::de::Visitor<'de> for VisitorMock {
        type Value = &'de str;

        fn visit_enum<V>(self, _variant: V) -> Result<Self::Value, serde::de::Error>
        where
            V: serde::de::EnumAccess<'de>,
        {
            Ok("valid_enum")
        }
    }

    let input = serde_json::json!({"variant_name": "variant_value"});
    let result: Result<_, serde::de::Error> = input.deserialize_enum("variant_name", &["variant_name"], VisitorMock);
    assert_eq!(result, Ok("valid_enum"));
}

fn test_deserialize_enum_empty_input() {
    struct VisitorMock;
    impl<'de> serde::de::Visitor<'de> for VisitorMock {
        type Value = &'de str;

        fn visit_enum<V>(self, _variant: V) -> Result<Self::Value, serde::de::Error>
        where
            V: serde::de::EnumAccess<'de>,
        {
            Ok("valid_enum")
        }
    }

    let input = serde_json::json!({});
    let result: Result<_, serde::de::Error> = input.deserialize_enum("variant_name", &["variant_name"], VisitorMock);
    assert!(result.is_err());
}

fn test_deserialize_enum_multiple_keys() {
    struct VisitorMock;
    impl<'de> serde::de::Visitor<'de> for VisitorMock {
        type Value = &'de str;

        fn visit_enum<V>(self, _variant: V) -> Result<Self::Value, serde::de::Error>
        where
            V: serde::de::EnumAccess<'de>,
        {
            Ok("valid_enum")
        }
    }

    let input = serde_json::json!({"variant_name": "variant_value", "extra_key": "extra_value"});
    let result: Result<_, serde::de::Error> = input.deserialize_enum("variant_name", &["variant_name"], VisitorMock);
    assert!(result.is_err());
}

