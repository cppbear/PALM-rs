// Answer 0

#[test]
fn test_next_element_seed_none() {
    use serde::de::{DeserializeSeed, Deserializer};
    use serde_json::Error;

    struct EmptyIterator;

    impl Iterator for EmptyIterator {
        type Item = &'static str;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = ();

        fn deserialize<T>(self, _deserializer: T) -> Result<Self::Value, serde::de::Error>
        where
            T: Deserializer<'de>,
        {
            Err(serde::de::Error::custom("Should not be called"))
        }
    }

    let mut iter = EmptyIterator;
    let result: Result<Option<()>, Error> = next_element_seed(&mut iter, TestSeed);
    assert_eq!(result, Ok(None));
}

