// Answer 0

#[test]
fn test_next_value_seed() {
    struct TestDeserializer<'de> {
        values: Vec<&'de str>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for TestDeserializer<'de> {
        type Error = TestError;

        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            Ok(None)
        }

        fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            if self.index < self.values.len() {
                let value = self.values[self.index];
                self.index += 1;
                seed.deserialize(value)
            } else {
                Err(TestError)
            }
        }
    }

    struct TestError;

    impl Error for TestError {
        fn custom<T>(msg: T) -> Self 
        where
            T: Display,
        {
            TestError
        }
    }

    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = &'de str;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            Ok(Box::leak(s.into_boxed_str()))
        }
    }

    let mut deserializer = TestDeserializer {
        values: vec!["value1", "value2"],
        index: 0,
    };

    let value: Result<&str, TestError> = deserializer.next_value_seed(TestSeed);
    assert_eq!(value.unwrap(), "value1");

    let value: Result<&str, TestError> = deserializer.next_value_seed(TestSeed);
    assert_eq!(value.unwrap(), "value2");

    let value: Result<&str, TestError> = deserializer.next_value_seed(TestSeed);
    assert!(value.is_err());
}

