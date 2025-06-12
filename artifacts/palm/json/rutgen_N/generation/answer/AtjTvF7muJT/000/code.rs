// Answer 0

#[test]
fn test_deserialize_string_success() {
    use serde::de::{self, Visitor};
    use serde_json::Deserializer;
    use std::io;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_owned())
        }
    }

    let input = r#""Hello, world!""#;
    let deserializer = Deserializer::from_str(input);
    let result: Result<String, _> = deserializer.deserialize_string(TestVisitor);

    assert_eq!(result.unwrap(), "Hello, world!");
}

#[test]
#[should_panic]
fn test_deserialize_string_invalid() {
    use serde::de::{self, Visitor};
    use serde_json::Deserializer;
    use std::io;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_owned())
        }
    }

    let input = r#"[1, 2, 3]"#; // Invalid input (not a string)
    let deserializer = Deserializer::from_str(input);

    // This should panic due to the input being a non-string
    let _result: Result<String, _> = deserializer.deserialize_string(TestVisitor);
}

