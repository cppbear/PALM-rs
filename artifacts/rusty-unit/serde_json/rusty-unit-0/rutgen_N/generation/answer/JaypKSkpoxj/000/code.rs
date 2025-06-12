// Answer 0

#[test]
fn test_deserialize_enum_valid() {
    use serde::de::{Deserializer, Visitor};
    use serde_json::Value;
    use serde_json::Deserializer as JsonDeserializer;
    use serde_json::Error;

    struct MyVisitor;

    impl<'de> Visitor<'de> for MyVisitor {
        type Value = &'de str;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string representing an enum variant")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value)
        }
    }

    let json_data = r#"{"key":"VariantA"}"#;
    let deserializer = JsonDeserializer::from_str(json_data);
    let visitor = MyVisitor;
    let variants = &["VariantA", "VariantB", "VariantC"];
    let result: Result<&str, Error> = deserializer.deserialize_enum("MyEnum", variants, visitor);

    assert_eq!(result, Ok("VariantA"));
}

#[test]
#[should_panic]
fn test_deserialize_enum_invalid() {
    use serde::de::{Deserializer, Visitor};
    use serde_json::Value;
    use serde_json::Deserializer as JsonDeserializer;
    use serde_json::Error;

    struct MyVisitor;

    impl<'de> Visitor<'de> for MyVisitor {
        type Value = &'de str;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string representing an enum variant")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value)
        }
    }

    let json_data = r#"{"key":"InvalidVariant"}"#;
    let deserializer = JsonDeserializer::from_str(json_data);
    let visitor = MyVisitor;
    let variants = &["VariantA", "VariantB", "VariantC"];
    let _result: Result<&str, Error> = deserializer.deserialize_enum("MyEnum", variants, visitor).unwrap();
}

