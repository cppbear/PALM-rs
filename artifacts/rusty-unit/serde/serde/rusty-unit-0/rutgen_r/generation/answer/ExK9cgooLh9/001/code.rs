// Answer 0

#[derive(Debug)]
struct ContentRefDeserializer;

impl ContentRefDeserializer {
    fn new(value: String) -> Self {
        ContentRefDeserializer
    }
}

struct TestSeed;

impl serde::de::DeserializeSeed<'_> for TestSeed {
    type Value = String;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'_>,
    {
        // Simulate successful deserialization
        Ok("deserialized_value".to_string())
    }
}

struct TestDeserializer {
    pending_content: Option<String>,
}

impl TestDeserializer {
    fn new(pending_content: Option<String>) -> Self {
        TestDeserializer { pending_content }
    }

    fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, serde::de::Error>
    where
        T: serde::de::DeserializeSeed<'_>,
    {
        match self.pending_content.take() {
            Some(value) => seed.deserialize(ContentRefDeserializer::new(value)),
            None => Err(serde::de::Error::custom("value is missing")),
        }
    }
}

#[test]
fn test_next_value_seed_success() {
    let mut deserializer = TestDeserializer::new(Some("test_content".to_string()));
    let seed = TestSeed;

    let result = deserializer.next_value_seed(seed).unwrap();
    assert_eq!(result, "deserialized_value");
}

#[test]
#[should_panic(expected = "value is missing")]
fn test_next_value_seed_none() {
    let mut deserializer = TestDeserializer::new(None);
    let seed = TestSeed;

    // This should panic due to the "None" case
    let _ = deserializer.next_value_seed(seed).unwrap();
}

