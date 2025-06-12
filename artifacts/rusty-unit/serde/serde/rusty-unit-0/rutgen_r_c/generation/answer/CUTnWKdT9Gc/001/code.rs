// Answer 0

#[test]
fn test_next_key_seed_with_valid_pair() {
    use std::marker::PhantomData;
    use serde::de::DeserializeSeed;

    struct TestKeyDeserializer;
    struct TestValueDeserializer;

    impl<'de> DeserializeSeed<'de> for TestKeyDeserializer {
        type Value = String;
        fn deserialize<T>(self, deserializer: T) -> Result<Self::Value, T::Error>
        where
            T: serde::de::Deserializer<'de>,
        {
            let key: String = deserializer.deserialize_str(TestStrVisitor)?;
            Ok(key)
        }
    }

    impl<'de> DeserializeSeed<'de> for TestValueDeserializer {
        type Value = i32;
        fn deserialize<T>(self, deserializer: T) -> Result<Self::Value, T::Error>
        where
            T: serde::de::Deserializer<'de>,
        {
            let value: i32 = deserializer.deserialize_i32(TestI32Visitor)?;
            Ok(value)
        }
    }

    struct TestMapAccess<'de> {
        pairs: Vec<(String, i32)>,
        index: usize,
        lifetime: PhantomData<&'de ()>,
    }

    impl<'de> MapAccess<'de> for TestMapAccess<'de> {
        type Error = Box<str>;

        fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            self.next_key_seed(seed)
        }
        
        fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            self.next_value_seed(seed)
        }
        
        fn next_entry_seed<TK, TV>(
            &mut self,
            kseed: TK,
            vseed: TV,
        ) -> Result<Option<(TK::Value, TV::Value)>, Self::Error>
        where
            TK: DeserializeSeed<'de>,
            TV: DeserializeSeed<'de>,
        {
            self.next_entry_seed(kseed, vseed)
        }

        fn size_hint(&self) -> Option<usize> {
            Some(self.pairs.len() - self.index)
        }

        fn next_pair(&mut self) -> Option<(String, i32)> {
            if self.index < self.pairs.len() {
                let pair = self.pairs[self.index].clone();
                self.index += 1;
                Some(pair)
            } else {
                None
            }
        }
    }

    let pairs = vec![
        ("key1".to_string(), 42),
        ("key2".to_string(), 17),
    ];

    let mut access = TestMapAccess {
        pairs,
        index: 0,
        lifetime: PhantomData,
    };

    let result = access.next_key_seed(TestKeyDeserializer);
    assert_eq!(result.unwrap(), Some("key1".to_string()));

    let result = access.next_value_seed(TestValueDeserializer);
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_next_key_seed_with_no_pairs() {
    use std::marker::PhantomData;
    use serde::de::DeserializeSeed;

    struct TestKeyDeserializer;

    impl<'de> DeserializeSeed<'de> for TestKeyDeserializer {
        type Value = String;
        fn deserialize<T>(self, deserializer: T) -> Result<Self::Value, T::Error>
        where
            T: serde::de::Deserializer<'de>,
        {
            let key: String = deserializer.deserialize_str(TestStrVisitor)?;
            Ok(key)
        }
    }

    struct TestMapAccess<'de> {
        pairs: Vec<(String, i32)>,
        index: usize,
        lifetime: PhantomData<&'de ()>,
    }

    impl<'de> MapAccess<'de> for TestMapAccess<'de> {
        type Error = Box<str>;

        fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            self.next_key_seed(seed)
        }

        fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            self.next_value_seed(seed)
        }

        fn next_entry_seed<TK, TV>(
            &mut self,
            kseed: TK,
            vseed: TV,
        ) -> Result<Option<(TK::Value, TV::Value)>, Self::Error>
        where
            TK: DeserializeSeed<'de>,
            TV: DeserializeSeed<'de>,
        {
            self.next_entry_seed(kseed, vseed)
        }

        fn size_hint(&self) -> Option<usize> {
            Some(self.pairs.len() - self.index)
        }
        
        fn next_pair(&mut self) -> Option<(String, i32)> {
            None
        }
    }

    let pairs: Vec<(String, i32)> = vec![];

    let mut access = TestMapAccess {
        pairs,
        index: 0,
        lifetime: PhantomData,
    };

    let result = access.next_key_seed(TestKeyDeserializer);
    assert_eq!(result.unwrap(), None);
}

