// Answer 0

#[test]
fn test_next_value_seed_none() {
    struct DummySeed;

    impl<'de> DeserializeSeed<'de> for DummySeed {
        type Value = String;

        fn deserialize<D>(self, _deserializer: D) -> Result<Self::Value, serde::de::Error>
        where
            D: Deserializer<'de>,
        {
            Err(serde::de::Error::custom("should not be called"))
        }
    }

    let deserializer = MapRefDeserializer {
        iter: std::iter::empty().into_iter(),
        value: None,
    };

    let result: Result<String, Error> = deserializer.next_value_seed(DummySeed);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().to_string(), "value is missing");
}

