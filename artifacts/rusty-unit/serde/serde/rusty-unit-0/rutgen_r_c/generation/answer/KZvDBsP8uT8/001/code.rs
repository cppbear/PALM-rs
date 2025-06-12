// Answer 0

fn test_next_entry_seed_success() {
    struct TestKeySeed;
    struct TestValueSeed;
    
    impl<'de> de::DeserializeSeed<'de> for TestKeySeed {
        type Value = String;

        fn deserialize<T>(self, deserializer: T) -> Result<Self::Value, T::Error>
        where
            T: de::Deserializer<'de>,
        {
            deserializer.deserialize_str(&mut serde::de::value::StrDeserializer::new("test_key"))
        }
    }

    impl<'de> de::DeserializeSeed<'de> for TestValueSeed {
        type Value = i32;

        fn deserialize<T>(self, deserializer: T) -> Result<Self::Value, T::Error>
        where
            T: de::Deserializer<'de>,
        {
            deserializer.deserialize_i32(&mut serde::de::value::I32Deserializer::new(42))
        }
    }

    let input_data = vec![("test_key", 42)];
    let iterator = input_data.into_iter().map(|(k, v)| (k, v));
    let mut map_deserializer = MapDeserializer {
        iter: iterator.fuse(),
        value: None,
        count: 0,
        lifetime: PhantomData,
        error: PhantomData,
    };

    let key_seed = TestKeySeed;
    let value_seed = TestValueSeed;

    let result = map_deserializer.next_entry_seed(key_seed, value_seed);

    assert_eq!(result.unwrap(), Some(("test_key".to_string(), 42)));
}

#[should_panic]
fn test_next_entry_seed_key_deserialization_failure() {
    struct FailingKeySeed;

    impl<'de> de::DeserializeSeed<'de> for FailingKeySeed {
        type Value = String;

        fn deserialize<T>(self, deserializer: T) -> Result<Self::Value, T::Error>
        where
            T: de::Deserializer<'de>,
        {
            Err(de::Error::custom("key deserialization failed"))
        }
    }

    struct ValidValueSeed;

    impl<'de> de::DeserializeSeed<'de> for ValidValueSeed {
        type Value = i32;

        fn deserialize<T>(self, deserializer: T) -> Result<Self::Value, T::Error>
        where
            T: de::Deserializer<'de>,
        {
            deserializer.deserialize_i32(&mut serde::de::value::I32Deserializer::new(42))
        }
    }

    let input_data = vec![("test_key", 42)];
    let iterator = input_data.into_iter().map(|(k, v)| (k, v));
    let mut map_deserializer = MapDeserializer {
        iter: iterator.fuse(),
        value: None,
        count: 0,
        lifetime: PhantomData,
        error: PhantomData,
    };

    let key_seed = FailingKeySeed;
    let value_seed = ValidValueSeed;

    let _ = map_deserializer.next_entry_seed(key_seed, value_seed);
}

