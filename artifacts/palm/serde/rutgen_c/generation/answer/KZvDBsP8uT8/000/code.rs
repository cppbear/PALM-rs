// Answer 0

#[test]
fn test_next_entry_seed_with_valid_pair() {
    use std::iter;

    struct TestKey;
    struct TestValue;

    impl de::DeserializeSeed<'de> for TestKey {
        type Value = &'static str;

        fn deserialize<T>(self, _: T) -> Result<Self::Value, ()> {
            Ok("key")
        }
    }

    impl de::DeserializeSeed<'de> for TestValue {
        type Value = i32;

        fn deserialize<T>(self, _: T) -> Result<Self::Value, ()> {
            Ok(42)
        }
    }

    let data = vec![(1, 2)];
    let mut map_deserializer = MapDeserializer {
        iter: data.into_iter().fuse(),
        value: None,
        count: 0,
        lifetime: PhantomData,
        error: PhantomData,
    };

    let result = map_deserializer.next_entry_seed(TestKey, TestValue).unwrap();

    assert_eq!(result, Some(("key", 42)));
}

#[test]
fn test_next_entry_seed_with_no_pair() {
    use std::iter;

    struct TestKey;
    struct TestValue;

    impl de::DeserializeSeed<'de> for TestKey {
        type Value = &'static str;

        fn deserialize<T>(self, _: T) -> Result<Self::Value, ()> {
            Ok("key")
        }
    }

    impl de::DeserializeSeed<'de> for TestValue {
        type Value = i32;

        fn deserialize<T>(self, _: T) -> Result<Self::Value, ()> {
            Ok(42)
        }
    }

    let data: Vec<(i32, i32)> = vec![];
    let mut map_deserializer = MapDeserializer {
        iter: data.into_iter().fuse(),
        value: None,
        count: 0,
        lifetime: PhantomData,
        error: PhantomData,
    };

    let result = map_deserializer.next_entry_seed(TestKey, TestValue).unwrap();

    assert_eq!(result, None);
}

