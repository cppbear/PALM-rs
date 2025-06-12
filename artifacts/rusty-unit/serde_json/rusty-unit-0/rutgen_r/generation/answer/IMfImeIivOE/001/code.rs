// Answer 0

#[test]
fn test_serialize_field_error() {
    use serde::Serialize;
    use serde_json::{Value, Error};

    struct TestSerializer {
        map: std::collections::HashMap<String, Value>,
    }

    impl TestSerializer {
        fn new() -> Self {
            TestSerializer {
                map: std::collections::HashMap::new(),
            }
        }

        fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Error>
        where
            T: ?Sized + Serialize,
        {
            self.map.insert(String::from(key), tri!(serde_json::to_value(value)));
            Ok(())
        }
    }

    struct UnserializableType;

    impl Serialize for UnserializableType {
        // This implementation will intentionally cause a serialization error
        fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            Err(serde::ser::Error::custom("Serialization Error"))
        }
    }

    let mut serializer = TestSerializer::new();
    let result = serializer.serialize_field("key", &UnserializableType);
    assert!(result.is_err());
    if let Err(ref e) = result {
        assert_eq!(e.to_string(), "Serialization Error");
    }
}

