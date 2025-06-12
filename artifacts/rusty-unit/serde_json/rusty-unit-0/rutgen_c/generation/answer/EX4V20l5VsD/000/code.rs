// Answer 0

#[test]
fn test_next_value_seed_with_value() {
    struct DummySeed;

    impl<'de> DeserializeSeed<'de> for DummySeed {
        type Value = i32;

        fn deserialize<T>(self, value: T) -> Result<Self::Value, Error>
        where
            T: serde::de::Deserialize<'de>,
        {
            if let Value::Number(num) = value {
                num.as_i64().map(|n| n as i32).ok_or(Error::custom("Not a valid number"))
            } else {
                Err(Error::custom("Expected number"))
            }
        }
    }

    let value = Value::Number(Number::from(42));
    let mut deserializer = MapRefDeserializer {
        iter: vec![].into_iter(), // Empty for this test
        value: Some(&value),
    };

    let result: Result<i32, Error> = deserializer.next_value_seed(DummySeed);
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_next_value_seed_with_none() {
    struct DummySeed;

    impl<'de> DeserializeSeed<'de> for DummySeed {
        type Value = i32;

        fn deserialize<T>(self, _value: T) -> Result<Self::Value, Error>
        where
            T: serde::de::Deserialize<'de>,
        {
            Err(Error::custom("This should not be called"))
        }
    }

    let mut deserializer = MapRefDeserializer {
        iter: vec![].into_iter(), // Empty for this test
        value: None,
    };

    let result: Result<i32, Error> = deserializer.next_value_seed(DummySeed);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "value is missing");
}

