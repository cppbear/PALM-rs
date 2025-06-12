// Answer 0

#[test]
fn test_next_entry_with_valid_input() {
    struct TestMapAccess {
        entries: Vec<(String, i32)>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for TestMapAccess {
        type Error = std::fmt::Error;
        
        fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            if self.index < self.entries.len() {
                let key = &self.entries[self.index].0;
                let result = key.deserialize(seed)?;
                Ok(Some(result))
            } else {
                Ok(None)
            }
        }

        fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            if self.index < self.entries.len() {
                let value = &self.entries[self.index].1;
                let result = value.deserialize(seed)?;
                self.index += 1; // Move to next entry
                Ok(result)
            } else {
                Err(std::fmt::Error)
            }
        }
    }

    let mut access = TestMapAccess {
        entries: vec![("one".to_string(), 1), ("two".to_string(), 2)],
        index: 0,
    };

    let entry = access.next_entry::<String, i32>().unwrap().unwrap();
    assert_eq!(entry, ("one".to_string(), 1));

    let entry = access.next_entry::<String, i32>().unwrap().unwrap();
    assert_eq!(entry, ("two".to_string(), 2));

    let entry = access.next_entry::<String, i32>().unwrap();
    assert_eq!(entry, None);
}

#[test]
fn test_next_entry_with_empty_map() {
    struct EmptyMapAccess {
        entries: Vec<(String, i32)>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for EmptyMapAccess {
        type Error = std::fmt::Error;

        fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            Ok(None) // Always return None, since the map is empty
        }

        fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            Err(std::fmt::Error) // There's no value to return
        }
    }

    let mut access = EmptyMapAccess {
        entries: vec![],
        index: 0,
    };

    let entry = access.next_entry::<String, i32>().unwrap();
    assert_eq!(entry, None);
}

