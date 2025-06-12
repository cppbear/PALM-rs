// Answer 0

#[test]
fn test_struct_variant_none_value() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("struct variant")
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(())
        }
    }

    struct TestDeserializer {
        value: Option<serde_json::Value>,
    }

    impl TestDeserializer {
        fn struct_variant<V>(
            self,
            _fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, serde::de::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            match self.value {
                Some(serde_json::Value::Object(v)) => v.deserialize_any(visitor),
                Some(other) => Err(serde::de::Error::invalid_type(
                    other.unexpected(),
                    &"struct variant",
                )),
                None => Err(serde::de::Error::invalid_type(
                    serde::de::Unexpected::UnitVariant,
                    &"struct variant",
                )),
            }
        }
    }

    let deserializer = TestDeserializer { value: None };
    let fields: &'static [&'static str] = &[];

    let result: Result<_, serde::de::Error> = deserializer.struct_variant(fields, TestVisitor);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.to_string(), "invalid type: unit variant, expected struct variant");
    }
}

