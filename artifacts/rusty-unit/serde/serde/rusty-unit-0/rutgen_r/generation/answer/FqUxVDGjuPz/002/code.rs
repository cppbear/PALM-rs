// Answer 0

#[derive(Debug)]
struct TestDeserializer;

impl<'de> de::DeserializeSeed<'de> for TestDeserializer {
    type Value = ();

    fn deserialize<Deserializer>(self, _: Deserializer) -> Result<Self::Value, de::Error> {
        Ok(())
    }
}

struct TestStruct {
    value: Option<()>,
}

impl TestStruct {
    fn new(value: Option<()>) -> Self {
        TestStruct { value }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, de::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.value {
            Some(value) => seed.deserialize(ContentDeserializer::new(value)),
            None => Err(de::Error::invalid_type(
                de::Unexpected::UnitVariant,
                &"newtype variant",
            )),
        }
    }
}

#[test]
fn test_newtype_variant_seed_none() {
    let test_struct = TestStruct::new(None);
    let deserializer = TestDeserializer;

    let result = test_struct.newtype_variant_seed(deserializer);

    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e, de::Error::invalid_type(de::Unexpected::UnitVariant, &"newtype variant"));
    }
}

