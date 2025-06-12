// Answer 0

#[test]
fn test_deserialize_enum_valid() {
    use serde::de::{self, Visitor};
    use serde_json::{Deserializer, Error};

    struct EnumVisitor;

    impl<'de> Visitor<'de> for EnumVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a valid enum variant")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }
    }

    let input = r#""variant1""#;
    let deserializer = Deserializer::from_str(input);
    let result: Result<String, Error> = deserializer.deserialize_enum("Enum", &["variant1", "variant2"], EnumVisitor);

    assert_eq!(result, Ok("variant1".to_string()));
}

#[test]
#[should_panic]
fn test_deserialize_enum_invalid_variant() {
    use serde::de::{self, Visitor};
    use serde_json::{Deserializer, Error};

    struct EnumVisitor;

    impl<'de> Visitor<'de> for EnumVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a valid enum variant")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }
    }

    let input = r#""invalid_variant""#;
    let deserializer = Deserializer::from_str(input);
    let result: Result<String, Error> = deserializer.deserialize_enum("Enum", &["variant1", "variant2"], EnumVisitor);

    // This will panic because "invalid_variant" is not in the expected variants
    assert!(result.is_err());
}

#[test]
fn test_deserialize_enum_empty_input() {
    use serde::de::{self, Visitor};
    use serde_json::{Deserializer, Error};

    struct EnumVisitor;

    impl<'de> Visitor<'de> for EnumVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a valid enum variant")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }
    }

    let input = r#""""#; // Empty string
    let deserializer = Deserializer::from_str(input);
    let result: Result<String, Error> = deserializer.deserialize_enum("Enum", &["variant1", "variant2"], EnumVisitor);

    assert!(result.is_err()); // Expect an error for empty input
}

