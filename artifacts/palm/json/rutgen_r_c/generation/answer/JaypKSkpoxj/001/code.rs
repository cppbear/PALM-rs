// Answer 0

#[test]
fn test_deserialize_enum_valid_variant() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = &'de str;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid enum variant")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value)
        }
    }

    let deserializer = MapKeyDeserializer {
        key: Cow::Borrowed("variant1"),
    };
    let result: Result<&str, Error> = deserializer.deserialize_enum("MyEnum", &["variant1", "variant2"], TestVisitor);
    assert_eq!(result.unwrap(), "variant1");
}

#[test]
fn test_deserialize_enum_invalid_variant() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = &'de str;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid enum variant")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Err(de::Error::custom("invalid visitor"))
        }
    }

    let deserializer = MapKeyDeserializer {
        key: Cow::Borrowed("variant3"),
    };
    let result: Result<&str, Error> = deserializer.deserialize_enum("MyEnum", &["variant1", "variant2"], TestVisitor);
    assert!(result.is_err());
}

#[should_panic(expected = "Expected enum variant not found")]
#[test]
fn test_deserialize_enum_panic_on_invalid_key() {
    struct PanicVisitor;

    impl<'de> Visitor<'de> for PanicVisitor {
        type Value = &'de str;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid enum variant")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if value == "invalid" {
                panic!("Expected enum variant not found");
            }
            Ok(value)
        }
    }

    let deserializer = MapKeyDeserializer {
        key: Cow::Borrowed("invalid"),
    };
    let _result: Result<&str, Error> = deserializer.deserialize_enum("MyEnum", &["variant1", "variant2"], PanicVisitor);
}

