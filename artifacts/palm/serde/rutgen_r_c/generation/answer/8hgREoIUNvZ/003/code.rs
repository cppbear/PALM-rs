// Answer 0

#[test]
fn test_next_entry_seed_success() {
    struct TestMapAccess {
        keys: Vec<String>,
        values: Vec<String>,
        current: usize,
    }

    impl<'de> MapAccess<'de> for TestMapAccess {
        type Error = TestError;

        fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            if self.current < self.keys.len() {
                let key = &self.keys[self.current];
                seed.deserialize_str(key).map(Some)
            } else {
                Ok(None)
            }
        }

        fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            if self.current < self.values.len() {
                let value = &self.values[self.current];
                seed.deserialize_str(value)
            } else {
                Err(TestError)
            }
        }
    }

    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = String;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_str(StrVisitor)
        }
    }

    struct StrVisitor;

    impl<'de> Visitor<'de> for StrVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_string())
        }
    }

    let mut test_map = TestMapAccess {
        keys: vec!["key1".to_string(), "key2".to_string()],
        values: vec!["value1".to_string(), "value2".to_string()],
        current: 0,
    };

    let kseed = TestSeed;
    let vseed = TestSeed;

    let result = test_map.next_entry_seed(kseed, vseed);

    assert_eq!(
        result,
        Ok(Some((String::from("key1"), String::from("value1"))))
    );
}

#[test]
fn test_next_entry_seed_no_more_keys() {
    struct TestMapAccess {
        keys: Vec<String>,
        values: Vec<String>,
        current: usize,
    }

    impl<'de> MapAccess<'de> for TestMapAccess {
        type Error = TestError;

        fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            if self.current < self.keys.len() {
                let key = &self.keys[self.current];
                seed.deserialize_str(key).map(Some)
            } else {
                Ok(None)
            }
        }

        fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            Err(TestError)
        }
    }

    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = String;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_str(StrVisitor)
        }
    }

    struct StrVisitor;

    impl<'de> Visitor<'de> for StrVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_string())
        }
    }

    let mut test_map = TestMapAccess {
        keys: vec!["key1".to_string(), "key2".to_string()],
        values: vec!["value1".to_string(), "value2".to_string()],
        current: 2,
    };

    let kseed = TestSeed;
    let vseed = TestSeed;

    let result = test_map.next_entry_seed(kseed, vseed);

    assert_eq!(result, Ok(None));
}

#[derive(Debug)]
struct TestError;

impl std::fmt::Display for TestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TestError")
    }
}

impl std::error::Error for TestError {}

