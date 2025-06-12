// Answer 0

#[derive(Debug)]
struct DummyVisitor {
    value: String,
}

impl<'de> de::Visitor<'de> for DummyVisitor {
    type Value = String;

    fn visit_enum<E>(self, _: E) -> Result<Self::Value, E::Error>
    where
        E: de::EnumAccess<'de>,
    {
        Ok(self.value)
    }
}

struct DummyDeserializer<'de> {
    input: &'de str,
}

impl<'de> DummyDeserializer<'de> {
    fn deserialize_enum<V>(
        self,
        name: &str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let _ = name;
        let _ = variants;
        visitor.visit_enum(self)
    }
}

impl<'de> de::EnumAccess<'de> for DummyDeserializer<'de> {
    type Error = serde::de::value::Error;

    fn variant_seed<V>(self, _seed: V) -> Result<(V::Value, Self), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        Ok((String::from("dummy_variant"), self))
    }
}

#[test]
fn test_deserialize_enum() {
    let deserializer = DummyDeserializer { input: "" };
    let variants = &["variant1", "variant2"];
    let visitor = DummyVisitor { value: String::from("dummy_variant") };

    let result: Result<String, _> = deserializer.deserialize_enum("TestEnum", variants, visitor);

    assert_eq!(result.unwrap(), "dummy_variant");
}

