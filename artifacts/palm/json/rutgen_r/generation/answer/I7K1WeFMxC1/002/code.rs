// Answer 0

#[test]
fn test_newtype_variant_seed_none() {
    use serde::de::{DeserializeSeed, Error, Unexpected};
    use serde::Deserialize;
    use std::marker::PhantomData;

    struct TestSeed {
        _marker: PhantomData<()>,
    }

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = ();

        fn deserialize<Deserializer>(self, _: Deserializer) -> Result<Self::Value, Error> {
            Err(Error::invalid_type(Unexpected::UnitVariant, &"newtype variant"))
        }
    }
    
    struct ValueStruct {
        value: Option<serde_json::Value>,
    }

    impl ValueStruct {
        fn new(value: Option<serde_json::Value>) -> Self {
            Self { value }
        }

        fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Error>
        where
            T: DeserializeSeed<'de>,
        {
            match self.value {
                Some(value) => seed.deserialize(value),
                None => Err(serde::de::Error::invalid_type(
                    Unexpected::UnitVariant,
                    &"newtype variant",
                )),
            }
        }
    }

    let value_struct = ValueStruct::new(None);
    let seed = TestSeed { _marker: PhantomData };

    let result = value_struct.newtype_variant_seed(seed);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid type: unit variant, expected newtype variant"
    );
}

