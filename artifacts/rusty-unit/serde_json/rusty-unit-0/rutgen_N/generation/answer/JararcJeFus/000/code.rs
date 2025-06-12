// Answer 0

#[test]
fn test_next_value_seed_success() {
    use serde::de::{DeserializeSeed, Deserializer, Error};

    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = String;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            String::deserialize(deserializer)
        }
    }

    struct TestValue<'de> {
        value: Option<String>,
    }

    impl<'de> TestValue<'de> {
        fn new(value: Option<String>) -> Self {
            TestValue { value }
        }

        fn take_value(&mut self) -> Option<String> {
            self.value.take()
        }
    }

    let test_value = TestValue::new(Some("test".to_string()));
    let mut value_ref = test_value;

    let result: Result<String, _> = value_ref.take_value().map(|v| TestSeed.deserialize(v));
    assert_eq!(result.unwrap(), "test");
}

#[test]
fn test_next_value_seed_failure() {
    use serde::de::{DeserializeSeed, Deserializer, Error};

    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = String;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            String::deserialize(deserializer)
        }
    }

    struct TestValue<'de> {
        value: Option<String>,
    }

    impl<'de> TestValue<'de> {
        fn new(value: Option<String>) -> Self {
            TestValue { value }
        }

        fn take_value(&mut self) -> Option<String> {
            self.value.take()
        }
    }

    let test_value = TestValue::new(None);
    let mut value_ref = test_value;

    let result: Result<String, _> = value_ref.take_value().map(|v| TestSeed.deserialize(v));
    assert!(result.is_err());
}

