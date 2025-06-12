// Answer 0

#[test]
fn test_next_key_seed_empty_iterator() {
    struct TestSeed;
    struct TestValue;

    impl serde::de::DeserializeSeed<'static> for TestSeed {
        type Value = TestValue;

        fn deserialize<T>(self, _deserializer: T) -> Result<Self::Value, T::Error>
        where
            T: serde::de::Deserializer<'static>,
        {
            Ok(TestValue)
        }
    }

    struct TestDeserializer<'de> {
        iter: std::slice::Iter<'de, Option<(String, String)>>,
    }

    impl<'de> TestDeserializer<'de> {
        fn new(entries: &'de [Option<(String, String)>]) -> Self {
            Self {
                iter: entries.iter(),
            }
        }

        fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, serde::de::Error>
        where
            T: serde::de::DeserializeSeed<'de>,
        {
            for entry in self.iter.by_ref() {
                if let Some((key, content)) = entry {
                    return seed.deserialize(ContentDeserializer::new(key)).map(Some);
                }
            }
            Ok(None)
        }
    }

    struct ContentDeserializer<'a> {
        key: &'a String,
    }

    impl<'a> ContentDeserializer<'a> {
        fn new(key: &'a String) -> Self {
            Self { key }
        }
    }

    let entries: Vec<Option<(String, String)>> = vec![];
    let mut deserializer = TestDeserializer::new(&entries);
    let result = deserializer.next_key_seed(TestSeed);

    assert_eq!(result, Ok(None));
}

