// Answer 0

#[test]
fn test_deserialize_enum_valid() {
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

    let key = Cow::from("variant1");
    let deserializer = MapKeyDeserializer { key };

    let result: Result<&str, Error> = deserializer.deserialize_enum("TestEnum", &["variant1", "variant2", "variant3"], TestVisitor);

    assert_eq!(result.unwrap(), "variant1");
}

#[test]
#[should_panic]
fn test_deserialize_enum_invalid_variant() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = &'de str;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid enum variant")
        }

        fn visit_str<E>(self, _value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            panic!("Invalid variant encountered");
        }
    }

    let key = Cow::from("invalid_variant");
    let deserializer = MapKeyDeserializer { key };

    let _result: Result<&str, Error> = deserializer.deserialize_enum("TestEnum", &["variant1", "variant2", "variant3"], TestVisitor);
}

#[test]
fn test_deserialize_enum_empty_variant_list() {
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

    let key = Cow::from("variant1");
    let deserializer = MapKeyDeserializer { key };

    let result: Result<&str, Error> = deserializer.deserialize_enum("TestEnum", &[], TestVisitor);

    assert!(result.is_err());
}

