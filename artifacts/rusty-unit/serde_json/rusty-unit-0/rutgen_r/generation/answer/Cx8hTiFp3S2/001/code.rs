// Answer 0

#[test]
fn test_struct_variant_invalid_type() {
    struct TestVisitor {
        value: Result<(), de::Error>,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct variant")
        }
        
        fn visit_unit_variant<E>(self, _name: &str) -> Result<Self::Value, E> {
            self.value.map_err(|e| de::Error::custom(e))
        }
    }

    let fields: &'static [&'static str] = &["field1", "field2"];
    let visitor = TestVisitor { value: Err(de::Error::custom("This should not be called.")) };

    let result = struct_variant(fields, visitor);
    
    assert!(result.is_err());
    match result {
        Err(de::Error::InvalidType { unexpected, .. }) => {
            assert_eq!(unexpected, Unexpected::UnitVariant);
        },
        _ => panic!("Expected an error of type InvalidType"),
    }
}

