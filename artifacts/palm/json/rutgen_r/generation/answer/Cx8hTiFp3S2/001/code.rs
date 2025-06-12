// Answer 0

#[test]
fn test_struct_variant_invalid_type() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct variant")
        }

        fn visit_unit_variant<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            // This function is not expected to be called; we are testing an error case
            Err(E::custom("should not have visited unit variant"))
        }
    }

    let fields: &'static [&'static str] = &["field1", "field2"];

    let result: Result<(), _> = struct_variant(fields, TestVisitor);

    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e, de::Error::invalid_type(
            Unexpected::UnitVariant,
            &"struct variant"
        ));
    }
}

