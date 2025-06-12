// Answer 0

#[test]
fn test_next_value_seed_valid() {
    use std::iter;

    struct KeyType;
    struct ValueType;

    impl de::DeserializeSeed<'static> for KeyType {
        type Value = String;
        fn deserialize<Deserializer>(self, _: Deserializer) -> Result<Self::Value, Box<str>> {
            Ok(String::from("key"))
        }
    }

    impl de::DeserializeSeed<'static> for ValueType {
        type Value = i32;
        fn deserialize<Deserializer>(self, _: Deserializer) -> Result<Self::Value, Box<str>> {
            Ok(42)
        }
    }

    let items = vec![("key1", 1), ("key2", 2)].into_iter();
    let mut map_deserializer = MapDeserializer {
        iter: items.fuse(),
        value: Some(ValueType),
        count: 0,
        lifetime: PhantomData,
        error: PhantomData,
    };

    let seed_key = KeyType;
    let seed_value = ValueType;

    let result = map_deserializer.next_entry_seed(seed_key, seed_value);

    assert!(result.is_ok());
    let entry = result.unwrap();
    assert!(entry.is_some());
    let (key, value) = entry.unwrap();
    assert_eq!(key, "key");
    assert_eq!(value, 42);
}

#[test]
#[should_panic(expected = "MapAccess::next_value called before next_key")]
fn test_next_value_seed_no_key() {
    struct NoKeyType;

    impl de::DeserializeSeed<'static> for NoKeyType {
        type Value = i32;
        fn deserialize<Deserializer>(self, _: Deserializer) -> Result<Self::Value, Box<str>> {
            Ok(42)
        }
    }

    let mut map_deserializer = MapDeserializer {
        iter: iter::empty(),
        value: None,
        count: 0,
        lifetime: PhantomData,
        error: PhantomData,
    };

    let seed_value = NoKeyType;

    // This should panic because there is no key set before calling next_value_seed
    let _ = map_deserializer.next_value_seed(seed_value);
}

