// Answer 0

#[test]
fn test_next_key_seed_none() {
    struct TestDeserializer;

    impl<'de> DeserializeSeed<'de> for TestDeserializer {
        type Value = usize;

        fn deserialize<D>(self, _: D) -> Result<Self::Value, serde::de::Error>
        where
            D: Deserializer<'de>,
        {
            Err(serde::de::Error::custom("not called"))
        }
    }

    struct TestMapDeserializer<'de> {
        iter: std::vec::IntoIter<(&'de str, &'de str)>,
        value: Option<&'de str>,
    }

    impl<'de> TestMapDeserializer<'de> {
        fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, serde::de::Error>
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

    let pairs: Vec<(&str, &str)> = vec![];
    let mut deserializer = TestMapDeserializer {
        iter: pairs.into_iter(),
        value: None,
    };

    let result: Result<Option<usize>, serde::de::Error> = deserializer.next_key_seed(TestDeserializer);

    assert_eq!(result, Ok(None));
}

