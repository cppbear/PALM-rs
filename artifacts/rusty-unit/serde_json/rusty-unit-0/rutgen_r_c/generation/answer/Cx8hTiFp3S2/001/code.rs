// Answer 0

#[test]
fn test_struct_variant_err_invalid_type() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("any value")
        }

        fn visit_unit<E>(self) -> result::Result<Self::Value, E> where E: de::Error {
            Ok(())
        }

        fn visit_newtype<T>(self, _value: T) -> result::Result<Self::Value, E>
        where
            T: de::Deserialize<'de>,
        {
            Ok(())
        }
    }

    let mut deserializer = Deserializer {
        read: SliceRead::new(&[]),
        scratch: vec![],
        remaining_depth: 10,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let result = UnitVariantAccess { de: &mut deserializer }
        .struct_variant(&["field1", "field2"], TestVisitor);

    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.to_string(), "invalid type: unit variant, expected a struct variant");
    }
}

