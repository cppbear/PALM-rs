// Answer 0

fn test_deserialize_enum_no_key() {
    struct VisitorImpl;

    impl<'de> serde::de::Visitor<'de> for VisitorImpl {
        type Value = ();

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an enum variant")
        }

        fn visit_enum<V>(
            self,
            _: V,
        ) -> Result<Self::Value, serde::de::Error>
        where
            V: serde::de::EnumAccess<'de>,
        {
            Ok(())
        }
    }

    let value = Value::Object(Map {
        map: MapImpl::new(), // Empty map to simulate no key
    });

    let result: Result<(), Error> = value.deserialize_enum("TestEnum", &["Variant1"], VisitorImpl);

    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e, serde::de::Error::invalid_value(
            Unexpected::Map,
            &"map with a single key",
        ));
    }
}

