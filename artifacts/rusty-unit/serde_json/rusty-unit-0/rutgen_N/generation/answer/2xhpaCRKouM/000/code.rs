// Answer 0

#[test]
fn test_serialize_field_map() {
    use serde::Serialize;
    use serde_json::ser::{self, SerializeMap};
    use serde_json::Compound;

    struct TestMap {
        entries: std::collections::HashMap<String, String>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                entries: std::collections::HashMap::new(),
            }
        }
    }

    impl SerializeMap for TestMap {
        type Ok = ();
        type Error = serde_json::Error;

        fn serialize_entry<K: Serialize, V: Serialize>(
            self,
            key: K,
            value: V,
        ) -> Result<Self::Ok, Self::Error> {
            let key = key.serialize(ser::Serializer::with_depth(0))?;
            let value = value.serialize(ser::Serializer::with_depth(0))?;
            self.entries.insert(key.to_string(), value.to_string());
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            // no operation for test
            Ok(())
        }
    }

    let mut compound = Compound::Map {
        ser: TestMap::new(),
    };
    
    let result = compound.serialize_field("test_key", &"test_value");
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_field_number_invalid_key() {
    use serde::Serialize;
    use serde_json::ser::{self, SerializeMap};
    use serde_json::Compound;

    struct TestNumber {
        ser: String,
    }

    impl TestNumber {
        fn new() -> Self {
            TestNumber {
                ser: String::new(),
            }
        }
    }

    impl Serialize for TestNumber {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.serialize_str(&self.ser)
        }
    }

    let mut compound = Compound::Number {
        ser: TestNumber::new(),
    };
    
    let _ = compound.serialize_field("invalid_key", &42); // This should panic
}

