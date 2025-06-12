// Answer 0

#[test]
fn test_next_value_seed_no_key() {
    struct DummyError;
    struct DummySeed;
    impl de::DeserializeSeed<'de> for DummySeed {
        type Value = usize;
        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, DummyError>
        where
            D: de::Deserializer<'de>,
        {
            Ok(0)
        }
    }

    struct TestIterator;
    impl Iterator for TestIterator {
        type Item = (usize, usize);
        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let mut map_deserializer: MapDeserializer<TestIterator, DummyError> = MapDeserializer {
        iter: TestIterator.fuse(),
        value: None,
        count: 0,
        lifetime: PhantomData,
        error: PhantomData,
    };

    let seed = DummySeed;
    let _ = map_deserializer.next_value_seed(seed);
}

#[test]
#[should_panic(expected = "MapAccess::next_value called before next_key")]
fn test_next_value_seed_with_no_previous_key() {
    struct DummyError;
    struct DummySeed;
    impl de::DeserializeSeed<'de> for DummySeed {
        type Value = usize;
        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, DummyError>
        where
            D: de::Deserializer<'de>,
        {
            Ok(1)
        }
    }

    struct TestIterator;
    impl Iterator for TestIterator {
        type Item = (usize, usize);
        fn next(&mut self) -> Option<Self::Item> {
            Some((0, 1))
        }
    }

    let mut map_deserializer: MapDeserializer<TestIterator, DummyError> = MapDeserializer {
        iter: TestIterator.fuse(),
        value: None,
        count: 0,
        lifetime: PhantomData,
        error: PhantomData,
    };

    let seed = DummySeed;
    let _ = map_deserializer.next_value_seed(seed);
}

