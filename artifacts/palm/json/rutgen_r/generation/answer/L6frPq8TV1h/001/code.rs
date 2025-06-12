// Answer 0

#[test]
fn test_newtype_variant_seed() {
    struct DummySeed;

    impl<'de> de::DeserializeSeed<'de> for DummySeed {
        type Value = ();

        fn deserialize<N>(self, _: N) -> Result<Self::Value>
        where
            N: de::Deserializer<'de>,
        {
            // This function won't be called as we're testing the error type directly.
            Ok(())
        }
    }

    let seed = DummySeed;
    let result: Result<(), _> = newtype_variant_seed(seed);

    assert!(result.is_err());

    if let Err(e) = result {
        match e {
            de::Error::InvalidType {
                unexpected,
                ..
            } => {
                assert_eq!(unexpected, Unexpected::UnitVariant);
            }
            _ => panic!("Expected de::Error::invalid_type but got a different error"),
        }
    } else {
        panic!("Expected an error, but got Ok");
    }
}

