// Answer 0

#[derive(Debug)]
struct EnumDeserializer<'de> {
    value: &'de str,
}

impl<'de> EnumDeserializer<'de> {
    fn new(value: &'de str) -> Self {
        EnumDeserializer { value }
    }
}

impl<'de> serde::de::Deserializer<'de> for EnumDeserializer<'de> {
    type Error = serde::de::value::Error;

    fn deserialize_enum<V>(
        self,
        name: &str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let _ = name;
        let _ = variants;
        visitor.visit_enum(self)
    }
}

#[derive(Debug)]
struct TestVisitor;

impl<'de> serde::de::Visitor<'de> for TestVisitor {
    type Value = &'de str;

    fn visit_enum<E>(self, _: E) -> Result<Self::Value, serde::de::value::Error>
    where
        E: serde::de::EnumAccess<'de>,
    {
        Ok("test_variant")
    }
}

#[test]
fn test_deserialize_enum() {
    let deserializer = EnumDeserializer::new("test_variant");
    let variants = &["variant1", "variant2", "variant3"];
    
    let result: Result<&str, serde::de::value::Error> = deserializer.deserialize_enum("TestEnum", variants, TestVisitor);
    
    assert_eq!(result.unwrap(), "test_variant");
}

