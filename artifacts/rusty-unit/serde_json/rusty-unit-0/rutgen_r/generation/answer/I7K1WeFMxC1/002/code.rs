// Answer 0

#[test]
fn test_newtype_variant_seed_with_none_value() {
    use serde::de::{DeserializeSeed, Error as DeserializeError, Unexpected};
    use serde_json::Error;

    struct NoneSeed;

    impl<'de> DeserializeSeed<'de> for NoneSeed {
        type Value = ();

        fn deserialize<D>(self, _: D) -> Result<Self::Value, DeserializeError>
        where
            D: serde::Deserializer<'de>,
        {
            Err(DeserializeError::custom("Should not be called"))
        }
    }

    struct TestStruct {
        value: Option<()>,
    }

    let test_struct = TestStruct { value: None };
    let seed = NoneSeed;

    let result: Result<(), Error> = test_struct.newtype_variant_seed(seed).map(|_| ());

    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error, Error::invalid_type(Unexpected::UnitVariant, &"newtype variant"));
    }
}

