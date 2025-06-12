// Answer 0

#[test]
fn test_deserialize_enum_no_variant_found() {
    struct TestVisitor;

    impl serde::de::Visitor<'static> for TestVisitor {
        type Value = ();

        fn visit_enum<V>(self, _visitor: V) -> Result<Self::Value, serde::de::Error>
        where
            V: serde::de::EnumAccess<'static>,
        {
            Ok(())
        }
    }

    struct DummyDeserializer<'de>(&'de [&'de str]);

    impl<'de> serde::Deserializer<'de> for DummyDeserializer<'de> {
        type Error = serde::de::Error;

        fn deserialize_enum<V>(
            self,
            name: &'static str,
            variants: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            for entry in self.0 {
                if let Some((key, _value)) = flat_map_take_entry(entry, variants) {
                    return visitor.visit_enum(EnumDeserializer::new(key, None));
                }
            }

            Err(serde::de::Error::custom(format_args!(
                "no variant of enum {} found in flattened data",
                name
            )))
        }
    }

    fn flat_map_take_entry(entry: &str, variants: &'static [&'static str]) -> Option<(&'static str, &'static str)> {
        if variants.contains(&entry) {
            Some((entry, "some_value"))
        } else {
            None
        }
    }

    struct EnumDeserializer<'a> {
        key: &'a str,
        value: Option<&'a str>,
    }

    impl<'a> EnumDeserializer<'a> {
        fn new(key: &'a str, value: Option<&'a str>) -> Self {
            Self { key, value }
        }
    }

    let deserializer = DummyDeserializer(&["non_variant_key"]);
    let result: Result<(), serde::de::Error> = deserializer.deserialize_enum("TestEnum", &["variant1", "variant2"], TestVisitor);

    assert!(result.is_err());
    assert_eq!(result.err().unwrap().to_string(), "no variant of enum TestEnum found in flattened data");
}

