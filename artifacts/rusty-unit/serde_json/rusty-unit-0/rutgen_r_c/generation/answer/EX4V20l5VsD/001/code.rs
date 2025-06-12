// Answer 0

fn test_next_value_seed_with_some_value() {
    use serde::de::{Deserializer, DeserializeSeed};
    use serde_json::Value;
    use std::collections::HashMap;

    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = HashMap<String, i32>;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            HashMap::deserialize(deserializer)
        }
    }

    let value = Value::Object(Map::from([
        (String::from("key1"), Value::Number(Number::from(1))),
        (String::from("key2"), Value::Number(Number::from(2))),
    ]));

    let mut deserializer = MapRefDeserializer {
        iter: value.as_object().unwrap().iter(),
        value: Some(&value),
    };

    let seed = TestSeed;
    let result: Result<HashMap<String, i32>, _> = deserializer.next_value_seed(seed);
    assert!(result.is_ok());
}

fn test_next_value_seed_with_none_value() {
    use serde::de::{Deserializer, DeserializeSeed};
    use serde_json::Value;
    use std::collections::HashMap;

    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = HashMap<String, i32>;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            HashMap::deserialize(deserializer)
        }
    }

    let value = Value::Object(Map::new());

    let mut deserializer = MapRefDeserializer {
        iter: value.as_object().unwrap().iter(),
        value: None,
    };

    let seed = TestSeed;
    let result: Result<HashMap<String, i32>, _> = deserializer.next_value_seed(seed);
    assert!(result.is_err());
}

fn test_next_value_seed_with_empty_map() {
    use serde::de::{Deserializer, DeserializeSeed};
    use serde_json::Value;
    use std::collections::HashMap;

    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = HashMap<String, i32>;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            HashMap::deserialize(deserializer)
        }
    }

    let value = Value::Object(Map::new());

    let mut deserializer = MapRefDeserializer {
        iter: value.as_object().unwrap().iter(),
        value: None,
    };

    let seed = TestSeed;
    let result: Result<HashMap<String, i32>, _> = deserializer.next_value_seed(seed);
    assert!(result.is_err());
}

fn test_next_value_seed_with_multiple_entries() {
    use serde::de::{Deserializer, DeserializeSeed};
    use serde_json::Value;
    use std::collections::HashMap;

    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = HashMap<String, i32>;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            HashMap::deserialize(deserializer)
        }
    }

    let value = Value::Object(Map::from([
        (String::from("key1"), Value::Number(Number::from(1))),
        (String::from("key2"), Value::Number(Number::from(2))),
    ]));

    let mut deserializer = MapRefDeserializer {
        iter: value.as_object().unwrap().iter(),
        value: Some(&value),
    };

    let seed = TestSeed;

    // Reading first value
    let result1: Result<HashMap<String, i32>, _> = deserializer.next_value_seed(seed);
    assert!(result1.is_ok());

    // Reading second value should still be ok as the value is not taken
    let result2: Result<HashMap<String, i32>, _> = deserializer.next_value_seed(seed);
    assert!(result2.is_err());
}

