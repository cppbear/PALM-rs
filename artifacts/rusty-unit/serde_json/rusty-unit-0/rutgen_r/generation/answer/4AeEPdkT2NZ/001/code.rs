// Answer 0

#[test]
fn test_deserialize_enum_invalid_type_other() {
    use serde::de::{self, Visitor, Deserialize};
    use serde_json::{Value, Error};

    struct DummyVisitor;

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an enum variant")
        }

        fn visit_enum<E>(self, _deserializer: E) -> Result<Self::Value, Error>
        where
            E: de::EnumAccess<'de>,
        {
            Ok(())
        }
    }

    // Instantiate `Value` with neither Object nor String (testing another variant),
    // for example, a Number
    let value = Value::Number(serde_json::Number::from(42));
    let result: Result<(), Error> = value.deserialize_enum("TestEnum", &["Variant1", "Variant2"], DummyVisitor);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().classify(), de::ErrorKind::InvalidType);
}

#[test]
fn test_deserialize_enum_invalid_type_null() {
    use serde::de::{self, Visitor, Deserialize};
    use serde_json::{Value, Error};

    struct DummyVisitor;

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an enum variant")
        }

        fn visit_enum<E>(self, _deserializer: E) -> Result<Self::Value, Error>
        where
            E: de::EnumAccess<'de>,
        {
            Ok(())
        }
    }

    // Instantiate `Value` with null
    let value = Value::Null;
    let result: Result<(), Error> = value.deserialize_enum("TestEnum", &["Variant1", "Variant2"], DummyVisitor);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().classify(), de::ErrorKind::InvalidType);
}

