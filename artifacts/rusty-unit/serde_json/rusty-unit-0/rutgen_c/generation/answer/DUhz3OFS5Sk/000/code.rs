// Answer 0

#[test]
fn test_unit_variant_struct_variant_err() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("struct variant")
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(())
        }
    }

    let unit_only = UnitOnly;
    let fields: &'static [&'static str] = &[];

    let result: Result<(), _> = unit_only.struct_variant(fields, TestVisitor);
    assert!(result.is_err());
}

