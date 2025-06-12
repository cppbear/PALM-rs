// Answer 0

#[test]
fn test_next_entry_with_some_entries() {
    use serde::de::{MapAccess, Deserialize, Deserializer};
    use serde::de::value::MapAccessDeserializer;
    use std::marker::PhantomData;
    
    struct TestMapAccess {
        entries: Vec<(String, i32)>,
        index: usize,
    }
    
    impl TestMapAccess {
        fn new(entries: Vec<(String, i32)>) -> Self {
            Self { entries, index: 0 }
        }
    }
    
    impl<'de> MapAccess<'de> for TestMapAccess {
        type Error = serde::de::value::Error;

        fn next_key_seed<K>(
            &mut self,
            seed: K
        ) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            if self.index < self.entries.len() {
                let key = &self.entries[self.index].0;
                self.index += 1;
                seed.deserialize_str(key.as_str())
            } else {
                Ok(None)
            }
        }

        fn next_entry_seed<K, V>(
            &mut self,
            key_seed: K,
            value_seed: V,
        ) -> Result<Option<(K::Value, V::Value)>, Self::Error>
        where
            K: DeserializeSeed<'de>,
            V: DeserializeSeed<'de>,
        {
            if self.index > 0 && self.index - 1 < self.entries.len() {
                let (key, value) = &self.entries[self.index - 1];
                let deserialized_key = key_seed.deserialize_str(key)?;
                let deserialized_value = value_seed.deserialize_i32(*value)?;
                Ok(Some((deserialized_key, deserialized_value)))
            } else {
                Ok(None)
            }
        }
    }
    
    let entries = vec![("key1".to_string(), 1), ("key2".to_string(), 2)];
    let mut access = TestMapAccess::new(entries);
    
    let result_1 = access.next_entry::<String, i32>();
    assert_eq!(result_1.unwrap(), Some(("key1".to_string(), 1)));

    let result_2 = access.next_entry::<String, i32>();
    assert_eq!(result_2.unwrap(), Some(("key2".to_string(), 2)));

    let result_3 = access.next_entry::<String, i32>();
    assert_eq!(result_3.unwrap(), None);
}

#[test]
fn test_next_entry_with_no_entries() {
    use serde::de::{MapAccess, Deserialize, Deserializer};
    use serde::de::value::MapAccessDeserializer;
    use std::marker::PhantomData;

    struct EmptyMapAccess;
    
    impl<'de> MapAccess<'de> for EmptyMapAccess {
        type Error = serde::de::value::Error;

        fn next_key_seed<K>(
            &mut self,
            _: K
        ) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            Ok(None)
        }

        fn next_entry_seed<K, V>(
            &mut self,
            _: K,
            _: V,
        ) -> Result<Option<(K::Value, V::Value)>, Self::Error>
        where
            K: DeserializeSeed<'de>,
            V: DeserializeSeed<'de>,
        {
            Ok(None)
        }
    }

    let mut access = EmptyMapAccess;

    let result = access.next_entry::<String, i32>();
    assert_eq!(result.unwrap(), None);
}

