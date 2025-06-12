// Answer 0

fn test_next_key_seed_success() {
    struct DummySeed;

    impl<'de> DeserializeSeed<'de> for DummySeed {
        type Value = String;

        fn deserialize<T>(self, _deserializer: T) -> Result<Self::Value, Error>
        where
            T: Deserializer<'de>,
        {
            Ok("dummy_key".to_string())
        }
    }

    struct TestIter {
        items: Vec<(&'static str, &'static str)>,
        index: usize,
    }

    impl Iterator for TestIter {
        type Item = (&'static str, &'static str);

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.items.len() {
                let item = self.items[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    struct TestDeserializer<'de> {
        iter: TestIter,
        value: Option<&'static str>,
    }

    impl<'de> TestDeserializer<'de> {
        fn new(items: Vec<(&'static str, &'static str)>) -> Self {
            TestDeserializer {
                iter: TestIter { items, index: 0 },
                value: None,
            }
        }

        fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error>
        where
            T: DeserializeSeed<'de>,
        {
            match self.iter.next() {
                Some((key, value)) => {
                    self.value = Some(value);
                    let key_de = MapKeyDeserializer {
                        key: Cow::Borrowed(&**key),
                    };
                    seed.deserialize(key_de).map(Some)
                }
                None => Ok(None),
            }
        }
    }

    let mut deserializer = TestDeserializer::new(vec![("key1", "value1")]);
    let result: Result<Option<String>, Error> = deserializer.next_key_seed(DummySeed);
    assert_eq!(result, Ok(Some("dummy_key".to_string())));
}

fn test_next_key_seed_none() {
    struct DummySeed;

    impl<'de> DeserializeSeed<'de> for DummySeed {
        type Value = String;

        fn deserialize<T>(self, _deserializer: T) -> Result<Self::Value, Error>
        where
            T: Deserializer<'de>,
        {
            Ok("dummy_key".to_string())
        }
    }

    struct TestIter {
        items: Vec<(&'static str, &'static str)>,
        index: usize,
    }

    impl Iterator for TestIter {
        type Item = (&'static str, &'static str);

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    struct TestDeserializer<'de> {
        iter: TestIter,
        value: Option<&'static str>,
    }

    impl<'de> TestDeserializer<'de> {
        fn new(items: Vec<(&'static str, &'static str)>) -> Self {
            TestDeserializer {
                iter: TestIter { items, index: 0 },
                value: None,
            }
        }

        fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error>
        where
            T: DeserializeSeed<'de>,
        {
            match self.iter.next() {
                Some((key, value)) => {
                    self.value = Some(value);
                    let key_de = MapKeyDeserializer {
                        key: Cow::Borrowed(&**key),
                    };
                    seed.deserialize(key_de).map(Some)
                }
                None => Ok(None),
            }
        }
    }

    let mut deserializer = TestDeserializer::new(vec![]);
    let result: Result<Option<String>, Error> = deserializer.next_key_seed(DummySeed);
    assert_eq!(result, Ok(None));
}

