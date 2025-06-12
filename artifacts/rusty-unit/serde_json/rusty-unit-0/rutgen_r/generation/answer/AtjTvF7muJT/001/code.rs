// Answer 0

#[test]
fn test_deserialize_string_success() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
            Ok(value.to_string())
        }
    }

    let input = "test_string";
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let result: Result<String, _> = deserializer.deserialize_string(TestVisitor);
    assert_eq!(result.unwrap(), "test_string");
}

#[test]
#[should_panic]
fn test_deserialize_string_invalid() {
    struct InvalidVisitor;

    impl<'de> de::Visitor<'de> for InvalidVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, _value: &str) -> Result<Self::Value, E> {
            panic!("This visitor is invalid")
        }
    }

    let input = r#""valid_string""#;
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let _result: Result<String, _> = deserializer.deserialize_string(InvalidVisitor);
}

