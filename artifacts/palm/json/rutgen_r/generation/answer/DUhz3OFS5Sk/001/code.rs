// Answer 0

#[test]
fn test_struct_variant_invalid_type() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("struct variant")
        }

        fn visit_unit_variant<E>(
            self,
            _: &'static str,
        ) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Err(E::invalid_type(
                Unexpected::UnitVariant,
                &"struct variant",
            ))
        }
    }

    let result: Result<(), de::Error> = struct_variant(
        &["field1", "field2"],
        TestVisitor,
    );
    
    assert!(result.is_err());
    match result {
        Err(de::Error::InvalidType { unexpected, expected }) => {
            assert_eq!(unexpected, Unexpected::UnitVariant);
            assert_eq!(expected, "struct variant");
        }
        _ => panic!("Expected an error of type de::Error::invalid_type"),
    }
}

