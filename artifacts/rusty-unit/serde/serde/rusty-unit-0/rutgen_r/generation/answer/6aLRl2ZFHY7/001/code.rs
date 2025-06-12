// Answer 0

#[derive(Debug)]
struct ContentDeserializer {
    value: String,
}

impl ContentDeserializer {
    fn new(value: String) -> Self {
        Self { value }
    }
}

struct TestSeed;

impl<'de> DeserializeSeed<'de> for TestSeed {
    type Value = String;

    fn deserialize<T>(&self, deserializer: T) -> Result<Self::Value, T::Error>
    where
        T: Deserializer<'de>,
    {
        let value: String = Deserialize::deserialize(deserializer)?;
        Ok(value)
    }
}

#[derive(Default)]
struct TestStruct {
    pending_content: Option<String>,
}

impl TestStruct {
    fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, String>
    where
        T: DeserializeSeed<'de>,
    {
        match self.pending_content.take() {
            Some(value) => seed.deserialize(ContentDeserializer::new(value)),
            None => Err("value is missing".to_string()),
        }
    }
}

#[test]
fn test_next_value_seed_with_some_content() {
    let mut test_struct = TestStruct {
        pending_content: Some("test value".to_string()),
    };
    let seed = TestSeed;

    let result = test_struct.next_value_seed(seed).unwrap();
    
    assert_eq!(result, "test value");
}

#[test]
#[should_panic(expected = "value is missing")]
fn test_next_value_seed_with_none_content() {
    let mut test_struct = TestStruct::default();
    let seed = TestSeed;

    // This should panic since pending_content is None
    let _ = test_struct.next_value_seed(seed);
}

