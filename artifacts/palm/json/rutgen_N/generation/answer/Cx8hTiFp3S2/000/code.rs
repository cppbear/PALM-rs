// Answer 0

#[test]
fn test_struct_variant_invalid_type() {
    struct TestVisitor {
        value: Option<()>,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct variant")
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(())
        }
    }

    let visitor = TestVisitor { value: None };
    let result: Result<(), _> = struct_variant(&["field1", "field2"], visitor);
    assert!(result.is_err());
    if let Err(de::Error::invalid_type(unexpected, _)) = result {
        assert_eq!(unexpected, Unexpected::UnitVariant);
    }
}

