// Answer 0

#[test]
fn test_tuple_variant_invalid_type() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a tuple variant")
        }

        fn visit_unit_variant<E>(self, _: &E) -> Result<Self::Value, E::Error>
        where
            E: de::EnumAccess<'de>,
        {
            Err(de::Error::custom("should not be unit variant"))
        }
    }

    let visitor = TestVisitor;
    let result: Result<(), de::Error> = tuple_variant(0, visitor);

    assert_eq!(result, Err(de::Error::invalid_type(
        de::Unexpected::UnitVariant,
        &"tuple variant",
    )));
}

#[test]
#[should_panic(expected = "should not be unit variant")]
fn test_tuple_variant_should_panic() {
    struct PanicVisitor;

    impl<'de> de::Visitor<'de> for PanicVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a tuple variant")
        }

        fn visit_unit_variant<E>(self, _: &E) -> Result<Self::Value, E::Error>
        where
            E: de::EnumAccess<'de>,
        {
            panic!("should not be unit variant");
        }
    }

    let visitor = PanicVisitor;
    let _result: Result<(), de::Error> = tuple_variant(0, visitor);
}

