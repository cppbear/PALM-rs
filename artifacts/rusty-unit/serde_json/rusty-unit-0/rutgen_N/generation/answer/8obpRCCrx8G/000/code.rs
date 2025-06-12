// Answer 0

#[test]
fn test_tuple_variant_error() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a tuple variant")
        }

        fn visit_unit(self) -> Result<Self::Value> {
            Ok(())
        }
    }

    let visitor = TestVisitor;
    let result: Result<(), _> = tuple_variant(0, visitor);
    assert!(result.is_err());
    
    if let Err(e) = result {
        assert_eq!(e, de::Error::invalid_type(Unexpected::UnitVariant, &"tuple variant"));
    }
}

