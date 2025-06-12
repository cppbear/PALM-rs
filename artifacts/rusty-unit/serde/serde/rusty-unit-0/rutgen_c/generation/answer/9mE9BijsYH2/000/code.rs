// Answer 0

#[test]
fn test_deserialize_enum_with_valid_visitor() {
    struct TestVisitor {
        value: Option<&'static str>,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = &'static str;

        fn visit_enum<E>(self, _: E) -> Result<Self::Value, E::Error>
        where
            E: de::EnumAccess<'de>,
        {
            Ok("test_variant")
        }
    }

    let deserializer = BorrowedStrDeserializer::<()>::new("test");
    let visitor = TestVisitor { value: None };

    let result = deserializer.deserialize_enum("TestEnum", &["test_variant"], visitor);
    assert_eq!(result.unwrap(), "test_variant");
}

#[test]
fn test_deserialize_enum_with_invalid_visitor() {
    struct InvalidVisitor;

    impl<'de> de::Visitor<'de> for InvalidVisitor {
        type Value = &'static str;

        fn visit_enum<E>(self, _: E) -> Result<Self::Value, E::Error>
        where
            E: de::EnumAccess<'de>,
        {
            Err(E::Error::custom("invalid visitor"))
        }
    }

    let deserializer = BorrowedStrDeserializer::<()>::new("test");
    let visitor = InvalidVisitor;

    let result = deserializer.deserialize_enum("TestEnum", &["test_variant"], visitor);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_enum_with_no_variants() {
    struct EmptyVisitor;

    impl<'de> de::Visitor<'de> for EmptyVisitor {
        type Value = ();

        fn visit_enum<E>(self, _: E) -> Result<Self::Value, E::Error>
        where
            E: de::EnumAccess<'de>,
        {
            Ok(())
        }
    }

    let deserializer = BorrowedStrDeserializer::<()>::new("test");
    let visitor = EmptyVisitor;

    let result = deserializer.deserialize_enum("EmptyEnum", &[], visitor);
    assert!(result.is_ok());
}

