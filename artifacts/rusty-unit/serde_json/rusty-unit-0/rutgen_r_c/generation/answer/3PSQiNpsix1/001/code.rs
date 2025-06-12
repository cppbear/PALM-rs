// Answer 0

fn test_next_key_seed_some() {
    use serde::de::{Deserializer, DeserializeSeed};
    use alloc::collections::BTreeMap;

    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = String;

        fn deserialize<D>(self, _deserializer: D) -> Result<Self::Value, Error>
        where
            D: Deserializer<'de>,
        {
            Ok("test_key".to_string())
        }
    }

    let mut map: BTreeMap<String, Value> = BTreeMap::new();
    map.insert("key1".to_string(), Value::Bool(true));
    map.insert("key2".to_string(), Value::Number(Number::from(10)));

    let iter = map.into_iter();
    let mut deserializer = MapRefDeserializer {
        iter,
        value: None,
    };

    let result = deserializer.next_key_seed(TestSeed).unwrap();
    assert_eq!(result, Some("test_key".to_string()));
}

fn test_next_key_seed_none() {
    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = String;

        fn deserialize<D>(self, _deserializer: D) -> Result<Self::Value, Error>
        where
            D: Deserializer<'de>,
        {
            Ok("key".to_string())
        }
    }

    let map: BTreeMap<String, Value> = BTreeMap::new();

    let iter = map.into_iter();
    let mut deserializer = MapRefDeserializer {
        iter,
        value: None,
    };

    let result = deserializer.next_key_seed(TestSeed).unwrap();
    assert_eq!(result, None);
}

#[cfg(test)]
fn main() {
    test_next_key_seed_some();
    test_next_key_seed_none();
}

