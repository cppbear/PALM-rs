// Answer 0

#[test]
fn test_deserialize_string_valid() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_owned())
        }
    }

    let input = r#""Hello, world!""#; // a valid JSON string
    let mut deserializer = serde_json::Deserializer::from_str(input);

    let result: Result<String, _> = deserializer.deserialize_string(TestVisitor);
    assert_eq!(result.unwrap(), "Hello, world!");
}

#[test]
#[should_panic]
fn test_deserialize_string_invalid() {
    struct PanicVisitor;

    impl<'de> serde::de::Visitor<'de> for PanicVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_bytes<E>(self, _: &[u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            panic!("should not be called")
        }
    }

    let input = r#"[1, 2, 3]"#; // invalid JSON for string deserialization
    let mut deserializer = serde_json::Deserializer::from_str(input);

    let _: Result<String, _> = deserializer.deserialize_string(PanicVisitor);
}

#[test]
fn test_deserialize_empty_string() {
    struct EmptyStringVisitor;

    impl<'de> serde::de::Visitor<'de> for EmptyStringVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_owned())
        }
    }

    let input = r#""""#; // an empty JSON string
    let mut deserializer = serde_json::Deserializer::from_str(input);

    let result: Result<String, _> = deserializer.deserialize_string(EmptyStringVisitor);
    assert_eq!(result.unwrap(), "");
}

