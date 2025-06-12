// Answer 0

#[test]
fn test_tuple_variant() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a tuple variant")
        }

        fn visit_unit(self) -> Result<Self::Value, Error> {
            Ok(())
        }

        fn visit_newtype<E: de::DeserializeOwned>(self, _: E) -> Result<Self::Value, Error> {
            Ok(())
        }

        fn visit_seq<V>(self, _: V) -> Result<Self::Value, Error>
        where
            V: de::SeqAccess<'de>,
        {
            Err(Error)
        }

        fn visit_map<V>(self, _: V) -> Result<Self::Value, Error>
        where
            V: de::MapAccess<'de>,
        {
            Err(Error)
        }
    }

    let unit_only = UnitOnly;
    let result = unit_only.tuple_variant(0, TestVisitor);
    assert!(result.is_err());

    if let Err(ref e) = result {
        assert_eq!(format!("{}", e), "invalid type: unit variant, expected a tuple variant");
    }
}

