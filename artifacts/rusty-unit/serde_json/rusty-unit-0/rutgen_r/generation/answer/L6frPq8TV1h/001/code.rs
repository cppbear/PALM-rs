// Answer 0

fn test_newtype_variant_seed() {
    struct TestSeed;

    impl<'de> de::DeserializeSeed<'de> for TestSeed {
        type Value = ();

        fn deserialize<E>(self, _deserializer: E) -> Result<Self::Value, E::Error>
        where
            E: de::Deserializer<'de>,
        {
            // This implementation is never actually called as we expect an error.
            Ok(())
        }
    }

    let seed = TestSeed;
    let result: Result<(), _> = newtype_variant_seed(seed);

    match result {
        Err(de::Error::invalid_type(Unexpected::UnitVariant, &"newtype variant")) => {}
        _ => panic!("Expected an invalid_type error for a UnitVariant"),
    }
}

