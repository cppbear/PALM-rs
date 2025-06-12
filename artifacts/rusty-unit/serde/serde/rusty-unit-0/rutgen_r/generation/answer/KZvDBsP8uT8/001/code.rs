// Answer 0

#[test]
fn test_next_entry_seed_some_key_value_err() {
    use serde::de::{self, DeserializeSeed, Deserializer};
    use serde::de::value::MapAccessDeserializer;
    use std::marker::PhantomData;

    struct TestKeySeed {
        should_fail: bool,
    }

    struct TestValueSeed;

    impl<'de> DeserializeSeed<'de> for TestKeySeed {
        type Value = String;

        fn deserialize<DE>(self, deserializer: DE) -> Result<Self::Value, DE::Error>
        where
            DE: Deserializer<'de>,
        {
            if self.should_fail {
                Err(de::Error::custom("Key deserialization error"))
            } else {
                let key: String = String::deserialize(deserializer)?;
                Ok(key)
            }
        }
    }

    impl<'de> DeserializeSeed<'de> for TestValueSeed {
        type Value = String;

        fn deserialize<DE>(self, deserializer: DE) -> Result<Self::Value, DE::Error>
        where
            DE: Deserializer<'de>,
        {
            let value: String = String::deserialize(deserializer)?;
            Ok(value)
        }
    }

    struct TestMapAccess {
        calls: usize,
    }

    impl TestMapAccess {
        fn new(calls: usize) -> Self {
            TestMapAccess { calls }
        }
    }

    impl de::MapAccess<'_> for TestMapAccess {
        type Error = serde::de::Error;

        fn next_key_seed<K>(
            &mut self,
            _seed: K,
        ) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'_>,
        {
            if self.calls > 0 {
                self.calls -= 1;
                Ok(Some("key".into()))
            } else {
                Ok(None)
            }
        }

        fn next_value_seed<V>(
            &mut self,
            _seed: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'_>,
        {
            Ok("value".into())
        }
    }

    let mut map_access = TestMapAccess::new(1);
    let kseed = TestKeySeed { should_fail: true };
    let vseed = TestValueSeed;

    let result: Result<Option<(String, String)>, _> =
        map_access.next_entry_seed(kseed, vseed);

    assert!(result.is_err());
}

