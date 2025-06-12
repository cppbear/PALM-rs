// Answer 0

#[test]
fn test_next_value_seed_success() {
    use serde::de::{DeserializeSeed, Error};
    use serde_json::Value;

    struct MockSeed;

    impl<'de> DeserializeSeed<'de> for MockSeed {
        type Value = Value;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Value::deserialize(deserializer)
        }
    }

    struct MockDeserializer {
        value: Option<Value>,
    }

    impl MockDeserializer {
        fn new(value: Option<Value>) -> Self {
            Self { value }
        }

        fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, serde::de::Error>
        where
            T: DeserializeSeed<'de>,
        {
            match self.value.take() {
                Some(value) => seed.deserialize(value),
                None => Err(serde::de::Error::custom("value is missing")),
            }
        }
    }

    let json_value = serde_json::json!({"key": "value"});
    let mut deserializer = MockDeserializer::new(Some(json_value.clone()));
    let seed = MockSeed;

    let result: Result<Value, _> = deserializer.next_value_seed(seed);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), json_value);
}

#[test]
fn test_next_value_seed_none() {
    use serde::de::{DeserializeSeed, Error};
    use serde_json::Value;

    struct MockSeed;

    impl<'de> DeserializeSeed<'de> for MockSeed {
        type Value = Value;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Value::deserialize(deserializer)
        }
    }

    struct MockDeserializer {
        value: Option<Value>,
    }

    impl MockDeserializer {
        fn new(value: Option<Value>) -> Self {
            Self { value }
        }

        fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, serde::de::Error>
        where
            T: DeserializeSeed<'de>,
        {
            match self.value.take() {
                Some(value) => seed.deserialize(value),
                None => Err(serde::de::Error::custom("value is missing")),
            }
        }
    }

    let mut deserializer = MockDeserializer::new(None);
    let seed = MockSeed;

    let result: Result<Value, _> = deserializer.next_value_seed(seed);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "value is missing");
}

