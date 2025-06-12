// Answer 0

#[cfg(test)]
fn test_variant_seed() {
    struct DummySeed;

    impl<'de> de::DeserializeSeed<'de> for DummySeed {
        type Value = bool;
        
        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            deserializer.deserialize_bool(DummyVisitor)
        }
    }

    struct DummyVisitor;

    impl<'de> de::Visitor<'de> for DummyVisitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean")
        }

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
            Ok(value)
        }
    }
    
    let value: CowStrDeserializer<'_, ()> = CowStrDeserializer {
        value: Cow::Borrowed("true"),
        marker: PhantomData,
    };

    let result: Result<(bool, UnitOnly<()>,), ()> = value.variant_seed(DummySeed);
    assert_eq!(result.unwrap().0, true);
}

