// Answer 0

#[test]
fn test_next_entry_seed() {
    use std::marker::PhantomData;

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
                self.index += 1;
                Ok(Some(self.data[self.index - 1].0.clone()))
            } else {
                Ok(None)
            }
        }

        fn next_value_seed<V>(&mut self, _seed: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            if self.index > 0 && self.index <= self.data.len() {
                Ok(self.data[self.index - 1].1)
            } else {
                Err(Error::custom("no value found"))
            }
        }
    }

    struct StringSeed;

    impl<'de> DeserializeSeed<'de> for StringSeed {
        type Value = String;

        fn deserialize<D>(self, _deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok("key".to_owned())
        }
    }

    struct IntSeed;

    impl<'de> DeserializeSeed<'de> for IntSeed {
        type Value = i32;

        fn deserialize<D>(self, _deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(42)
        }
    }

    let mut access = TestMapAccess {
        data: vec![("key1".to_owned(), 1), ("key2".to_owned(), 2)],
        index: 0,
    };

    let key_seed = StringSeed;
    let value_seed = IntSeed;

    let entry1 = access.next_entry_seed(key_seed, value_seed).unwrap();
    assert_eq!(entry1, Some(("key1".to_owned(), 1)));

    let entry2 = access.next_entry_seed(key_seed, value_seed).unwrap();
    assert_eq!(entry2, Some(("key2".to_owned(), 2)));

    let entry3 = access.next_entry_seed(key_seed, value_seed).unwrap();
    assert_eq!(entry3, None);
}

