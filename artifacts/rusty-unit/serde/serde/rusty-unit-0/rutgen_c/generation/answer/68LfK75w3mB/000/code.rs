// Answer 0

#[test]
fn test_next_key_with_some_key() {
    struct TestMapAccess {
        keys: Vec<String>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for TestMapAccess {
        type Error = Error;

        fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            if self.index < self.keys.len() {
                let key = &self.keys[self.index];
                self.index += 1;
                Ok(Some(seed.deserialize(&mut StringDeserializer::new(key))?))
            } else {
                Ok(None)
            }
        }

        fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            Err(Error::custom("not implemented"))
        }
    }

    struct StringDeserializer<'de>(&'de str);
    
    impl<'de> DeserializeSeed<'de> for StringDeserializer<'de> {
        type Value = String;
        
        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(self.0.to_string())
        }
    }

    let mut map_access = TestMapAccess {
        keys: vec!["key1".to_string(), "key2".to_string()],
        index: 0,
    };

    assert_eq!(map_access.next_key::<StringDeserializer>(), Ok(Some("key1".to_string())));
    assert_eq!(map_access.next_key::<StringDeserializer>(), Ok(Some("key2".to_string())));
    assert_eq!(map_access.next_key::<StringDeserializer>(), Ok(None));
}

#[test]
fn test_next_key_with_none() {
    struct EmptyMapAccess;

    impl<'de> MapAccess<'de> for EmptyMapAccess {
        type Error = Error;

        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            Ok(None)
        }

        fn next_value_seed<V>(&mut self, _seed: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            Err(Error::custom("not implemented"))
        }
    }

    let mut map_access = EmptyMapAccess;

    assert_eq!(map_access.next_key::<StringDeserializer>(), Ok(None));
}

