// Answer 0

#[test]
fn test_newtype_variant_seed() {
    struct MySeed;

    impl<'de> de::DeserializeSeed<'de> for MySeed {
        type Value = ();
        
        fn deserialize<D>(self, _deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            unimplemented!()
        }
    }

    let seed = MySeed;
    let result: Result<(), serde::de::Error> = newtype_variant_seed(seed);
    assert!(result.is_err());

    if let Err(e) = result {
        assert_eq!(e, serde::de::Error::invalid_type(
            serde::de::Unexpected::UnitVariant,
            &"newtype variant",
        ));
    }
}

