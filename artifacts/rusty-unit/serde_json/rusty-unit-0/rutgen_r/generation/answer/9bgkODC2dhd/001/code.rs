// Answer 0

fn test_newtype_variant_seed_valid() {
    use serde::de::DeserializeSeed;
    use serde::Serialize;
    use serde_json::Error;
    use serde_json::{Deserializer, Value};

    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = String;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let value: Value = Value::deserialize(deserializer)?;
            Ok(value.as_str().unwrap_or("").to_owned())
        }
    }

    struct NewtypeVariant {
        value: Option<Value>,
    }

    let input_value = Value::String("test".to_string());
    let newtype_variant = NewtypeVariant {
        value: Some(input_value.clone()),
    };

    let result: Result<String, Error> = newtype_variant.newtype_variant_seed(TestSeed);

    assert_eq!(result, Ok("test".to_string()));
}

fn test_newtype_variant_seed_none() {
    use serde::de::DeserializeSeed;
    use serde_json::Error;
    use serde_json::{Deserializer, Value};

    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = String;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let value: Value = Value::deserialize(deserializer)?;
            Ok(value.as_str().unwrap_or("").to_owned())
        }
    }

    struct NewtypeVariant {
        value: Option<Value>,
    }

    let newtype_variant = NewtypeVariant {
        value: None,
    };

    let result: Result<String, Error> = newtype_variant.newtype_variant_seed(TestSeed);

    assert!(result.is_err());
}

