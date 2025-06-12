// Answer 0

#[test]
fn test_deserialize_any_with_valid_string() {
    use serde::de::{Deserializer, Visitor, Deserialize};
    use std::string::ToString;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
            Ok(value)
        }
    }

    let key_deserializer = MapKeyDeserializer {
        key: Cow::Borrowed("test_key"),
    };

    let result: Result<String, Error> = key_deserializer.deserialize_any(TestVisitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "test_key");
}

#[test]
fn test_deserialize_any_with_invalid_type() {
    use serde::de::{Deserializer, Visitor, Deserialize};

    struct InvalidVisitor;

    impl<'de> Visitor<'de> for InvalidVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an invalid type")
        }

        fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> {
            Err(de::Error::invalid_value(Unexpected::Bool(true), &self))
        }
    }

    let key_deserializer = MapKeyDeserializer {
        key: Cow::Borrowed("test_key"),
    };

    let result: Result<String, Error> = key_deserializer.deserialize_any(InvalidVisitor);
    assert!(result.is_err());
}

