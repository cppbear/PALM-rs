// Answer 0

#[test]
fn test_next_entry_seed_some() {
    use serde::de::{self, DeserializeSeed, IntoDeserializer};
    use std::marker::PhantomData;

    struct MockKeySeed;
    struct MockValueSeed;

    impl<'de> DeserializeSeed<'de> for MockKeySeed {
        type Value = String;

        fn deserialize<T>(self, _: T) -> std::result::Result<Self::Value, Self::Error>
        where
            T: de::Deserializer<'de>,
        {
            Ok("key".to_string())
        }
    }

    impl<'de> DeserializeSeed<'de> for MockValueSeed {
        type Value = String;

        fn deserialize<T>(self, _: T) -> std::result::Result<Self::Value, Self::Error>
        where
            T: de::Deserializer<'de>,
        {
            Ok("value".to_string())
        }
    }

    struct MockDeserializer {
        pairs: Vec<(String, String)>,
        index: usize,
    }

    impl MockDeserializer {
        fn new(pairs: Vec<(String, String)>) -> Self {
            MockDeserializer { pairs, index: 0 }
        }

        fn next_pair(&mut self) -> Option<(String, String)> {
            if self.index < self.pairs.len() {
                let pair = self.pairs[self.index].clone();
                self.index += 1;
                Some(pair)
            } else {
                None
            }
        }
    }

    let mut deserializer = MockDeserializer::new(vec![("key".to_string(), "value".to_string())]);
    let result = deserializer.next_entry_seed(MockKeySeed, MockValueSeed).unwrap();
    assert_eq!(result, Some((String::from("key"), String::from("value"))));
}

#[test]
fn test_next_entry_seed_none() {
    use serde::de::{self, DeserializeSeed, IntoDeserializer};

    struct MockKeySeed;
    struct MockValueSeed;

    impl<'de> DeserializeSeed<'de> for MockKeySeed {
        type Value = String;

        fn deserialize<T>(self, _: T) -> std::result::Result<Self::Value, Self::Error>
        where
            T: de::Deserializer<'de>,
        {
            Ok("key".to_string())
        }
    }

    impl<'de> DeserializeSeed<'de> for MockValueSeed {
        type Value = String;

        fn deserialize<T>(self, _: T) -> std::result::Result<Self::Value, Self::Error>
        where
            T: de::Deserializer<'de>,
        {
            Ok("value".to_string())
        }
    }

    struct MockDeserializer {
        pairs: Vec<(String, String)>,
        index: usize,
    }

    impl MockDeserializer {
        fn new(pairs: Vec<(String, String)>) -> Self {
            MockDeserializer { pairs, index: 0 }
        }

        fn next_pair(&mut self) -> Option<(String, String)> {
            if self.index < self.pairs.len() {
                let pair = self.pairs[self.index].clone();
                self.index += 1;
                Some(pair)
            } else {
                None
            }
        }
    }

    let mut deserializer = MockDeserializer::new(vec![]);
    let result = deserializer.next_entry_seed(MockKeySeed, MockValueSeed).unwrap();
    assert_eq!(result, None);
}

