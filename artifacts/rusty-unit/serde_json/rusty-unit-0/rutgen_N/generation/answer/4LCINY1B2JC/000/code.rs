// Answer 0

#[test]
fn test_deserialize_string_valid() {
    use serde::de::{self, Visitor};
    use serde_json::Value;
    
    struct StringVisitor {
        result: Option<String>,
    }
    
    impl<'de> Visitor<'de> for StringVisitor {
        type Value = String;

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value)
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }
    }
    
    let value = Value::String("test".to_string());
    let visitor = StringVisitor { result: None };
    let result: Result<String, _> = value.deserialize_string(visitor);
    
    assert_eq!(result, Ok("test".to_string()));
}

#[test]
fn test_deserialize_string_invalid() {
    use serde::de::{self, Visitor};
    use serde_json::Value;
    
    struct StringVisitor;

    impl<'de> Visitor<'de> for StringVisitor {
        type Value = String;

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value)
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }
    }

    let value = Value::Bool(true); // Not a string, valid test case
    let visitor = StringVisitor;
    let result: Result<String, _> = value.deserialize_string(visitor);
    
    assert!(result.is_err());
}

