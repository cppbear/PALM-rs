// Answer 0

fn tuple_variant_test() {
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
            Err(de::Error::custom("unexpected unit"))
        }
    }

    let result: Result<(), _> = tuple_variant(0, TestVisitor);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), de::Error::invalid_type(
        Unexpected::UnitVariant,
        &"tuple variant",
    ));
}

