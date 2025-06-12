// Answer 0

#[test]
fn test_tuple_variant_invalid_type() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a tuple variant")
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Err(E::custom("unexpected unit variant"))
        }
    }

    let result: Result<(), _> = tuple_variant(0, TestVisitor);
    assert!(result.is_err());
    match result {
        Err(de::Error::InvalidType { .. }) => {}
        _ => panic!("Expected an invalid type error"),
    }
}

