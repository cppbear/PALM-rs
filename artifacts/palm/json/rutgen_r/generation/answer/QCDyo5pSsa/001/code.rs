// Answer 0

fn test_newtype_variant_seed() {
    struct TestSeed;

    impl<'de> de::DeserializeSeed<'de> for TestSeed {
        type Value = ();

        fn deserialize<DES>(self, _deserializer: DES) -> Result<Self::Value, de::Error>
        where
            DES: de::Deserializer<'de>,
        {
            Err(de::Error::invalid_type(
                de::Unexpected::UnitVariant,
                &"newtype variant",
            ))
        }
    }

    let seed = TestSeed;
    let result: Result<(), de::Error> = newtype_variant_seed(seed);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e, de::Error::invalid_type(de::Unexpected::UnitVariant, &"newtype variant"));
    }
}

