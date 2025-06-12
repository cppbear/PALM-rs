// Answer 0

#[derive(Debug)]
struct TestSeed;

impl<'de> serde::de::DeserializeSeed<'de> for TestSeed {
    type Value = i32;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: i32 = Deserialize::deserialize(deserializer)?;
        Ok(value)
    }
}

#[derive(Debug)]
struct MockDeserializer {
    value: Option<i32>,
}

impl MockDeserializer {
    fn new(value: Option<i32>) -> Self {
        Self { value }
    }

    fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, serde::de::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some(value) => seed.deserialize(value.into_deserializer()),
            None => Err(serde::de::Error::custom("value is missing")),
        }
    }
}

#[test]
fn test_next_value_seed_success() {
    let mut deserializer = MockDeserializer::new(Some(42));
    let seed = TestSeed;
    let result = deserializer.next_value_seed(seed);

    assert_eq!(result.unwrap(), 42);
}

#[test]
#[should_panic(expected = "value is missing")]
fn test_next_value_seed_none() {
    let mut deserializer = MockDeserializer::new(None);
    let seed = TestSeed;
    let _result = deserializer.next_value_seed(seed).expect("Expected an error");
}

