// Answer 0

#[derive(Debug)]
struct TestSeed;

impl serde::de::DeserializeSeed<'de> for TestSeed {
    type Value = ();

    fn deserialize<D>(self, _: D) -> Result<Self::Value, serde::de::Error>
    where
        D: serde::Deserializer<'de>,
    {
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

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, serde::de::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        match self.value {
            Some(value) => seed.deserialize(value),
            None => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::UnitVariant,
                &"newtype variant",
            )),
        }
    }
}

#[test]
fn test_newtype_variant_seed_none() {
    let test_struct = TestStruct::new(None);
    let seed = TestSeed;

    let result = test_struct.newtype_variant_seed(seed);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid type: unit variant, expected newtype variant"
    );
}

