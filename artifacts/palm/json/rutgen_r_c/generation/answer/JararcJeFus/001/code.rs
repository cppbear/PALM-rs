// Answer 0

fn test_next_value_seed_with_some_value() {
    struct MockSeed {
        expected_value: Value,
    }

    impl<'de> DeserializeSeed<'de> for MockSeed {
        type Value = Value;

        fn deserialize<D>(self, _deserializer: D) -> Result<Self::Value, Error>
        where
            D: Deserializer<'de>,
        {
            Ok(self.expected_value)
        }
    }

    let value = Value::String(String::from("test"));
    let mut map_deserializer = MapDeserializer {
        iter: vec![].into_iter(),
        value: Some(value.clone()),
    };

    let seed = MockSeed {
        expected_value: value,
    };

    let result = map_deserializer.next_value_seed(seed).unwrap();
    assert_eq!(result, value);
}

fn test_next_value_seed_with_none_value() {
    struct MockSeed;

    impl<'de> DeserializeSeed<'de> for MockSeed {
        type Value = Value;

        fn deserialize<D>(self, _deserializer: D) -> Result<Self::Value, Error>
        where
            D: Deserializer<'de>,
        {
            Err(Error::custom("This should not be called."))
        }
    }

    let mut map_deserializer = MapDeserializer {
        iter: vec![].into_iter(),
        value: None,
    };

    let seed = MockSeed;

    let result = map_deserializer.next_value_seed(seed);
    assert!(result.is_err());
    if let Err(ref e) = result {
        assert_eq!(e.to_string(), "value is missing");
    }
}

