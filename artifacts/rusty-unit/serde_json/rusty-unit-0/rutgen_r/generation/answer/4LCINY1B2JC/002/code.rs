// Answer 0

#[test]
fn test_deserialize_string_valid() {
    use serde::de::{self, Visitor};
    use serde_json::{Value, Error};

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value)
        }
    }

    let value = Value::String("test".to_string());
    let visitor = TestVisitor;

    let result: Result<String, Error> = value.deserialize_string(visitor);
    assert_eq!(result.unwrap(), "test".to_string());
}

#[test]
fn test_deserialize_string_invalid_type() {
    use serde::de::{self, Visitor};
    use serde_json::{Value, Error};

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_string<E>(self, _: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            panic!("visit_string should not be called for non-string value");
        }
    }

    let value = Value::Number(42.into());
    let visitor = TestVisitor;

    let result: Result<String, Error> = value.deserialize_string(visitor);
    assert!(result.is_err());
}

