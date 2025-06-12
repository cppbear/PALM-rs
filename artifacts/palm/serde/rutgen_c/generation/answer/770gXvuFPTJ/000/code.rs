// Answer 0

#[test]
fn test_next_key_seed() {
    struct MockDeserializer<'de> {
        keys: Vec<&'de str>,
        current_index: usize,
    }

    impl<'de> MockDeserializer<'de> {
        fn new(keys: Vec<&'de str>) -> Self {
            MockDeserializer {
                keys,
                current_index: 0,
            }
        }
    }

    impl<'de> MapAccess<'de> for MockDeserializer<'de> {
        type Error = Error;

        fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            if self.current_index < self.keys.len() {
                let key = self.keys[self.current_index];
                self.current_index += 1;
                Ok(Some(seed.deserialize(key)?))
            } else {
                Ok(None)
            }
        }

        // Other required methods in MapAccess can be empty for this test.
        fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error> {
            unimplemented!()
        }
    }

    struct StringSeed;

    impl<'de> DeserializeSeed<'de> for StringSeed {
        type Value = String;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(String::from(deserializer))
        }
    }

    let mut deserializer = MockDeserializer::new(vec!["key1", "key2"]);
    let seed = StringSeed;

    assert_eq!(deserializer.next_key_seed(seed).unwrap(), Some("key1".to_string()));
    assert_eq!(deserializer.next_key_seed(seed).unwrap(), Some("key2".to_string()));
    assert_eq!(deserializer.next_key_seed(seed).unwrap(), None);
}

#[test]
fn test_next_key_seed_no_keys() {
    struct MockDeserializer<'de> {
        keys: Vec<&'de str>,
        current_index: usize,
    }

    impl<'de> MockDeserializer<'de> {
        fn new(keys: Vec<&'de str>) -> Self {
            MockDeserializer {
                keys,
                current_index: 0,
            }
        }
    }

    impl<'de> MapAccess<'de> for MockDeserializer<'de> {
        type Error = Error;

        fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            if self.current_index < self.keys.len() {
                let key = self.keys[self.current_index];
                self.current_index += 1;
                Ok(Some(seed.deserialize(key)?))
            } else {
                Ok(None)
            }
        }

        fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error> {
            unimplemented!()
        }
    }

    struct StringSeed;

    impl<'de> DeserializeSeed<'de> for StringSeed {
        type Value = String;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(String::from(deserializer))
        }
    }

    let mut deserializer = MockDeserializer::new(vec![]);
    let seed = StringSeed;

    assert_eq!(deserializer.next_key_seed(seed).unwrap(), None);
}

