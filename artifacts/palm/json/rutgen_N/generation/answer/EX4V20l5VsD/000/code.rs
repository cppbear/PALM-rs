// Answer 0

#[test]
fn test_next_value_seed_success() {
    use serde::de::{DeserializeSeed, Deserializer, Error as DeError};
    use serde_json::Value;
    
    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = String;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            let value: Value = Value::deserialize(deserializer)?;
            value.as_str().map(|s| s.to_string()).ok_or(DeError::custom("Not a string"))
        }
    }

    struct TestDeserializer {
        value: Option<Value>,
    }

    impl TestDeserializer {
        fn new(value: Value) -> Self {
            Self { value: Some(value) }
        }

        fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, DeError>
        where
            T: DeserializeSeed<'de>,
        {
            match self.value.take() {
                Some(value) => seed.deserialize(value.into_deserializer()),
                None => Err(DeError::custom("value is missing")),
            }
        }
    }

    let json_value = Value::String("test".to_string());
    let mut deserializer = TestDeserializer::new(json_value);
    let result: Result<String, DeError> = deserializer.next_value_seed(TestSeed);

    assert_eq!(result.unwrap(), "test");
}

#[test]
fn test_next_value_seed_failure() {
    use serde::de::{DeserializeSeed, Deserializer, Error as DeError};
    use serde_json::Value;

    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = String;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            let value: Value = Value::deserialize(deserializer)?;
            value.as_str().map(|s| s.to_string()).ok_or(DeError::custom("Not a string"))
        }
    }

    struct TestDeserializer {
        value: Option<Value>,
    }

    impl TestDeserializer {
        fn new(value: Option<Value>) -> Self {
            Self { value }
        }

        fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, DeError>
        where
            T: DeserializeSeed<'de>,
        {
            match self.value.take() {
                Some(value) => seed.deserialize(value.into_deserializer()),
                None => Err(DeError::custom("value is missing")),
            }
        }
    }

    let mut deserializer = TestDeserializer::new(None);
    let result: Result<String, DeError> = deserializer.next_value_seed(TestSeed);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "value is missing");
}

