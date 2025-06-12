// Answer 0

#[test]
fn test_next_element_seed_some() {
    use serde::de::DeserializeSeed;
    use serde_json::Value;
    use std::iter::Once;

    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = Value;

        fn deserialize<T>(&self, value: T) -> Result<Self::Value, serde::de::Error>
        where
            T: serde::de::Deserialize<'de>,
        {
            Value::deserialize(value)
        }
    }

    let input: &str = r#"{"key": "value"}"#;
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let mut seed = TestSeed;

    let result = deserializer.next_element_seed(&mut seed).unwrap();
    assert!(result.is_some());
    assert_eq!(result.unwrap()["key"], "value");
}

#[test]
fn test_next_element_seed_none() {
    use serde::de::DeserializeSeed;
    use serde_json::Value;

    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = Value;

        fn deserialize<T>(&self, value: T) -> Result<Self::Value, serde::de::Error>
        where
            T: serde::de::Deserialize<'de>,
        {
            Value::deserialize(value)
        }
    }

    let input: &str = r#"[]"#;
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let mut seed = TestSeed;

    let result = deserializer.next_element_seed(&mut seed).unwrap();
    assert!(result.is_none());
}

