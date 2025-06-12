// Answer 0

#[test]
fn test_next_element_seed_some() {
    use serde::de::{DeserializeSeed, Deserializer};
    use serde_json::Value;
    use serde_json::error::Error;

    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = Value;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            Value::deserialize(deserializer)
        }
    }

    struct TestIterator {
        data: Vec<Value>,
        iter: std::slice::Iter<'static, Value>,
    }

    impl TestIterator {
        fn new(data: Vec<Value>) -> Self {
            let iter = data.iter();
            TestIterator { data, iter }
        }

        fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error>
        where
            T: DeserializeSeed<'de>,
        {
            match self.iter.next() {
                Some(value) => seed.deserialize(value).map(Some),
                None => Ok(None),
            }
        }
    }

    let data = vec![Value::String("test".to_string()), Value::Number(42.into())];
    let mut iter = TestIterator::new(data);

    let result: Result<Option<Value>, Error> = iter.next_element_seed(TestSeed);
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

#[test]
fn test_next_element_seed_none() {
    use serde::de::{DeserializeSeed, Deserializer};
    use serde_json::Value;
    use serde_json::error::Error;

    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = Value;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            Value::deserialize(deserializer)
        }
    }

    struct TestIterator {
        iter: std::slice::Iter<'static, Value>,
    }

    impl TestIterator {
        fn new(data: Vec<Value>) -> Self {
            let iter = data.iter();
            TestIterator { iter }
        }

        fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error>
        where
            T: DeserializeSeed<'de>,
        {
            match self.iter.next() {
                Some(value) => seed.deserialize(value).map(Some),
                None => Ok(None),
            }
        }
    }

    let data: Vec<Value> = vec![];
    let mut iter = TestIterator::new(data);

    let result: Result<Option<Value>, Error> = iter.next_element_seed(TestSeed);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
}

