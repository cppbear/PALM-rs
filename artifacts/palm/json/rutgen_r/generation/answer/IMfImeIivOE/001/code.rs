// Answer 0


#[cfg(test)]
use serde::Serialize;
use std::collections::HashMap;

struct JsonSerializer {
    map: HashMap<String, serde_json::Value>,
}

impl JsonSerializer {
    fn new() -> Self {
        JsonSerializer {
            map: HashMap::new(),
        }
    }

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), serde_json::Error>
    where
        T: ?Sized + Serialize,
    {
        self.map.insert(String::from(key), serde_json::to_value(value)?);
        Ok(())
    }
}

#[test]
fn test_serialize_field_with_invalid_value() {
    struct InvalidValue;

    impl Serialize for InvalidValue {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            // Intentionally causing serialization to fail
            Err(serde::ser::Error::custom("serialization error"))
        }
    }

    let mut serializer = JsonSerializer::new();
    let result = serializer.serialize_field("invalid_key", &InvalidValue);
    assert!(result.is_err());
}


