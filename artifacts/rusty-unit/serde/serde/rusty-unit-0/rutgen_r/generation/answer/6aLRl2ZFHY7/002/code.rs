// Answer 0

#[derive(Debug)]
struct ContentDeserializer;

impl ContentDeserializer {
    fn new(value: String) -> Self {
        ContentDeserializer
    }
}

#[derive(Debug)]
struct MockSeed;

impl<'de> DeserializeSeed<'de> for MockSeed {
    type Value = String;

    fn deserialize<S>(self, _: S) -> Result<Self::Value, Self::Error>
    where
        S: Deserializer<'de>,
    {
        Ok("Mocked Value".to_string())
    }
}

#[derive(Debug)]
struct Deserializer<'de> {
    pending_content: Option<String>,
}

impl<'de> Deserializer<'de> {
    fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.pending_content.take() {
            Some(value) => seed.deserialize(ContentDeserializer::new(value)),
            None => Err(Error::custom("value is missing")),
        }
    }
}

#[derive(Debug)]
struct Error {
    message: String,
}

impl Error {
    fn custom(msg: &str) -> Self {
        Error { message: msg.to_string() }
    }
}

#[test]
fn test_next_value_seed_none_pending_content() {
    let mut deserializer = Deserializer { pending_content: None };
    let seed = MockSeed;

    let result: Result<String, Error> = deserializer.next_value_seed(seed);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().message, "value is missing");
}

