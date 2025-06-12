// Answer 0

#[test]
fn test_next_element_seed_returns_ok_none_when_iter_is_empty() {
    use serde::de::{DeserializeSeed, Deserializer};
    use serde_json::Error;

    struct EmptyIterator;

    impl Iterator for EmptyIterator {
        type Item = serde_json::Value;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    struct TestDeserializer {
        iter: EmptyIterator,
    }

    impl TestDeserializer {
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

    struct DummySeed;

    impl<'de> DeserializeSeed<'de> for DummySeed {
        type Value = ();

        fn deserialize<D>(self, _deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(())
        }
    }

    let mut deserializer = TestDeserializer { iter: EmptyIterator };
    let seed = DummySeed;

    let result = deserializer.next_element_seed(seed);
    assert_eq!(result, Ok(None));
}

