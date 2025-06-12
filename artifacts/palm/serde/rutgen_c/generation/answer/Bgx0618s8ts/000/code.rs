// Answer 0

#[test]
fn test_struct_variant_invalid_type() {
    struct TestVisitor;
    
    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();
        
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("any value")
        }
        
        fn visit_unit(self) -> Result<Self::Value, Self::Error> {
            Ok(())
        }
        
        fn visit_str<E>(self, _value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Err(de::Error::invalid_value(
                Unexpected::Str("unexpected string"),
                &"a unit variant",
            ))
        }
    }

    let unit_only = UnitOnly::<de::Error> { marker: PhantomData };
    let fields: &'static [&'static str] = &[];

    let result: Result<(), de::Error> = unit_only.struct_variant(fields, TestVisitor);
    assert!(result.is_err());
}

