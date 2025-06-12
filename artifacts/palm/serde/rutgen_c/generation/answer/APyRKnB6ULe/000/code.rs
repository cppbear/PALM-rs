// Answer 0

#[test]
fn test_next_value_success() {
    struct TestMapAccess {
        data: Vec<(String, i32)>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for TestMapAccess {
        type Error = Error;

        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            if self.index < self.data.len() {
                let key = &self.data[self.index].0;
                // Simulate deserialization of key
                Ok(Some(key.clone()))
            } else {
                Ok(None)
            }
        }

        fn next_value_seed<V>(&mut self, _seed: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            if self.index < self.data.len() {
                let value = self.data[self.index].1;
                self.index += 1;
                // Simulate deserialization of value
                Ok(value)
            } else {
                Err(Error::custom("No more values"))
            }
        }
    }

    let mut map_access = TestMapAccess {
        data: vec![("key1".to_string(), 1), ("key2".to_string(), 2)],
        index: 0,
    };

    assert_eq!(map_access.next_value::<i32>().unwrap(), 1);
    assert_eq!(map_access.next_value::<i32>().unwrap(), 2);
}

#[test]
#[should_panic]
fn test_next_value_panics_if_no_key() {
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
            Err(Error::custom("No more values"))
        }
    }

    let mut map_access = EmptyMapAccess;
    map_access.next_value::<i32>(); // This should panic since no key was retrieved
}

#[test]
fn test_next_value_empty() {
    struct TestMapAccessEmpty {
        index: usize,
    }

    impl<'de> MapAccess<'de> for TestMapAccessEmpty {
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
            Err(Error::custom("No more values"))
        }
    }

    let mut map_access = TestMapAccessEmpty { index: 0 };
    assert!(map_access.next_value::<i32>().is_err());
}

