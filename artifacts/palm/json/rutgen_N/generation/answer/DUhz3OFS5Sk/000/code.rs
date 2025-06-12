// Answer 0

#[test]
fn test_struct_variant_invalid_type() {
    struct MyVisitor;

    impl<'de> de::Visitor<'de> for MyVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct variant")
        }

        fn visit_unit_variant<E>(self) -> Result<Self::Value, E> where E: de::Error {
            Err(E::invalid_value(de::Unexpected::UnitVariant, &"struct variant"))
        }
    }

    let fields: &'static [&'static str] = &["field1", "field2"];
    
    let result = struct_variant(fields, MyVisitor);
    assert!(result.is_err());
}

