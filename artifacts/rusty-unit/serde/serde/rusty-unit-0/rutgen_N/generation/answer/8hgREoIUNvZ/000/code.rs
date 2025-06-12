// Answer 0

#[test]
fn test_next_entry_seed_with_some_values() {
    use serde::de::{Deserializer, DeserializeSeed, MapAccess};
    use serde::Deserialize;
    use std::marker::PhantomData;

    struct TestMapAccess {
        items: Vec<(i32, String)>,
        current: usize,
    }

    impl TestMapAccess {
        fn new(items: Vec<(i32, String)>) -> Self {
            TestMapAccess { items, current: 0 }
        }
    }

    impl<'de> MapAccess<'de> for TestMapAccess {
        type Error = serde::de::value::Error;

        fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            if self.current < self.items.len() {
                let (key, _) = &self.items[self.current];
                let value = seed.deserialize(*key).map(|v| Some(v))?;
                Ok(value)
            } else {
                Ok(None)
            }
        }

        fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            if self.current < self.items.len() {
                let (_, value) = &self.items[self.current];
                let result = seed.deserialize(value.clone())?;
                self.current += 1;
                Ok(result)
            } else {
                Err(serde::de::value::Error::custom("No more values"))
            }
        }
    }

    struct IntSeed;
    
    impl<'de> DeserializeSeed<'de> for IntSeed {
        type Value = i32;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_i32(serde::de::value::BorrowedBytes::from(&0))
        }
    }

    struct StringSeed;

    impl<'de> DeserializeSeed<'de> for StringSeed {
        type Value = String;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok("test".to_string())
        }
    }

    let items = vec![(1, "one".to_string()), (2, "two".to_string())];
    let mut access = TestMapAccess::new(items);
    let kseed = IntSeed;
    let vseed = StringSeed;
    
    let result1 = access.next_entry_seed(kseed, vseed);
    assert!(result1.is_ok());
    assert_eq!(result1.unwrap(), Some((1, "test".to_string())));
    
    let result2 = access.next_entry_seed(kseed, vseed);
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), Some((2, "test".to_string())));
    
    let result3 = access.next_entry_seed(kseed, vseed);
    assert!(result3.is_ok());
    assert_eq!(result3.unwrap(), None);
}

#[test]
#[should_panic]
fn test_next_entry_seed_with_no_values() {
    use serde::de::{Deserializer, DeserializeSeed, MapAccess};

    struct EmptyMapAccess;

    impl<'de> MapAccess<'de> for EmptyMapAccess {
        type Error = serde::de::value::Error;

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
            Err(serde::de::value::Error::custom("No more values"))
        }
    }

    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = String;

        fn deserialize<D>(self, _deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok("value".to_string())
        }
    }

    let mut access = EmptyMapAccess;
    let kseed = TestSeed;
    let vseed = TestSeed;
    let _ = access.next_entry_seed(kseed, vseed);
}

