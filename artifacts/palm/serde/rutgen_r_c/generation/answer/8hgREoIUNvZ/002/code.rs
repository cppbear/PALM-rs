// Answer 0

#[test]
fn test_next_entry_seed_ok() {
    struct KeySeed;
    struct ValueSeed;

    impl<'de> DeserializeSeed<'de> for KeySeed {
        type Value = String;
        fn deserialize<D>(self, _deserializer: D) -> Result<Self::Value, D::Error>
        where D: Deserializer<'de> {
            Ok("key".to_string())
        }
    }

    impl<'de> DeserializeSeed<'de> for ValueSeed {
        type Value = String;
        fn deserialize<D>(self, _deserializer: D) -> Result<Self::Value, D::Error>
        where D: Deserializer<'de> {
            Ok("value".to_string())
        }
    }

    struct TestMapAccess<'de> {
        keys: Vec<String>,
        values: Vec<String>,
    }

    impl<'de> MapAccess<'de> for TestMapAccess<'de> {
        type Error = std::fmt::Error;

        fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
        where K: DeserializeSeed<'de> {
            if self.keys.is_empty() {
                Ok(None)
            } else {
                let key = self.keys.remove(0);
                seed.deserialize(&mut mock::Deserializer).map(Some)
            }
        }

        fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
        where V: DeserializeSeed<'de> {
            if self.values.is_empty() {
                Err(std::fmt::Error)
            } else {
                let value = self.values.remove(0);
                seed.deserialize(&mut mock::Deserializer)
            }
        }
    }

    let mut map_access = TestMapAccess {
        keys: vec!["key".to_string()],
        values: vec!["value".to_string()],
    };

    let result: Result<Option<(String, String)>, _> = map_access.next_entry_seed(KeySeed, ValueSeed);
    assert_eq!(result, Ok(Some(("key".to_string(), "value".to_string()))));
}

#[test]
fn test_next_entry_seed_no_key() {
    struct KeySeed;
    struct ValueSeed;

    impl<'de> DeserializeSeed<'de> for KeySeed {
        type Value = String;
        fn deserialize<D>(self, _deserializer: D) -> Result<Self::Value, D::Error>
        where D: Deserializer<'de> {
            Ok("key".to_string())
        }
    }

    impl<'de> DeserializeSeed<'de> for ValueSeed {
        type Value = String;
        fn deserialize<D>(self, _deserializer: D) -> Result<Self::Value, D::Error>
        where D: Deserializer<'de> {
            Ok("value".to_string())
        }
    }

    struct TestMapAccess<'de> {
        keys: Vec<String>,
        values: Vec<String>,
    }

    impl<'de> MapAccess<'de> for TestMapAccess<'de> {
        type Error = std::fmt::Error;

        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<K::Value>, Self::Error>
        where K: DeserializeSeed<'de> {
            Ok(None)
        }

        fn next_value_seed<V>(&mut self, _seed: V) -> Result<V::Value, Self::Error>
        where V: DeserializeSeed<'de> {
            Ok("value".to_string())
        }
    }

    let mut map_access = TestMapAccess {
        keys: vec![],
        values: vec![],
    };

    let result: Result<Option<(String, String)>, _> = map_access.next_entry_seed(KeySeed, ValueSeed);
    assert_eq!(result, Ok(None));
}

#[test]
fn test_next_entry_seed_key_error() {
    struct KeySeed;

    impl<'de> DeserializeSeed<'de> for KeySeed {
        type Value = String;
        fn deserialize<D>(self, _deserializer: D) -> Result<Self::Value, D::Error>
        where D: Deserializer<'de> {
            Ok("key".to_string())
        }
    }

    struct ValueSeed;

    impl<'de> DeserializeSeed<'de> for ValueSeed {
        type Value = String;
        fn deserialize<D>(self, _deserializer: D) -> Result<Self::Value, D::Error>
        where D: Deserializer<'de> {
            Err(std::fmt::Error)
        }
    }

    struct TestMapAccess<'de> {
        keys: Vec<String>,
        values: Vec<String>,
    }

    impl<'de> MapAccess<'de> for TestMapAccess<'de> {
        type Error = std::fmt::Error;

        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<K::Value>, Self::Error>
        where K: DeserializeSeed<'de> {
            if self.keys.is_empty() {
                Ok(None)
            } else {
                let key = self.keys.remove(0);
                Ok(Some(key))
            }
        }

        fn next_value_seed<V>(&mut self, _seed: V) -> Result<V::Value, Self::Error>
        where V: DeserializeSeed<'de> {
            Ok("value".to_string())
        }
    }

    let mut map_access = TestMapAccess {
        keys: vec!["key".to_string()],
        values: vec![],
    };

    let result: Result<Option<(String, String)>, _> = map_access.next_entry_seed(KeySeed, ValueSeed);
    assert_eq!(result, Err(std::fmt::Error));
}

