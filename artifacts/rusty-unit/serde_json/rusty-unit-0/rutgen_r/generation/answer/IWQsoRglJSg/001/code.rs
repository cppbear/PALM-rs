// Answer 0

#[test]
fn test_deserialize_any_with_valid_visitor() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a valid string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_string())
        }
    }

    let deserializer = BorrowedCowStrDeserializer::new("test_key");
    let result: Result<String, serde_json::Error> = deserializer.deserialize_any(TestVisitor);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "test_key");
}

#[test]
#[should_panic]
fn test_deserialize_any_with_invalid_visitor() {
    struct InvalidVisitor;

    impl<'de> serde::de::Visitor<'de> for InvalidVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("invalid visitor")
        }

        fn visit_str<E>(self, _value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            panic!("This visitor is invalid!");
        }
    }

    let deserializer = BorrowedCowStrDeserializer::new("test_key");
    let _result: Result<(), serde_json::Error> = deserializer.deserialize_any(InvalidVisitor);
}

#[test]
fn test_deserialize_any_with_empty_key() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a valid string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_string())
        }
    }

    let deserializer = BorrowedCowStrDeserializer::new("");
    let result: Result<String, serde_json::Error> = deserializer.deserialize_any(TestVisitor);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "");
}

