// Answer 0

#[test]
fn test_deserialize_enum_valid_case() {
    struct TestVisitor {
        value: Option<String>,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid enum value")
        }

        fn visit_enum<E>(self, _: E) -> Result<Self::Value, Error>
        where
            E: de::EnumAccess<'de>,
        {
            Ok("EnumVariant".to_string())
        }
    }

    let deserializer = BorrowedCowStrDeserializer {
        value: Cow::Borrowed("some input"),
    };

    let result = deserializer.deserialize_enum("TestEnum", &["EnumVariant"], TestVisitor { value: None });
    assert_eq!(result.unwrap(), "EnumVariant".to_string());
}

#[test]
#[should_panic]
fn test_deserialize_enum_invalid_case() {
    struct PanicVisitor;

    impl<'de> de::Visitor<'de> for PanicVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid enum value")
        }

        fn visit_enum<E>(self, _: E) -> Result<Self::Value, Error>
        where
            E: de::EnumAccess<'de>,
        {
            panic!();
        }
    }

    let deserializer = BorrowedCowStrDeserializer {
        value: Cow::Borrowed("some input"),
    };

    let _result = deserializer.deserialize_enum("TestEnum", &["EnumVariant"], PanicVisitor);
}

