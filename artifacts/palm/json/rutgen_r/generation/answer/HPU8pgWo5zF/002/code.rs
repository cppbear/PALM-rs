// Answer 0

fn test_serialize_object_success() {
    struct MockSerializer {
        map: Vec<(String, i32)>,
        error: Option<String>,
    }

    impl serde::Serializer for MockSerializer {
        type Ok = ();
        type Error = String;

        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn serialize_map(self, len: Option<usize>) -> Result<Self, Self::Error> {
            if self.error.is_none() {
                Ok(self)
            } else {
                Err(self.error.unwrap())
            }
        }
        
        fn serialize_entry<K, V>(&mut self, key: K, value: V) -> Result<(), Self::Error>
        where
            K: serde::Serialize,
            V: serde::Serialize,
        {
            if let Some(err) = &self.error {
                Err(err.clone())
            } else {
                self.map.push((key.serialize(&mut self)?.to_string(), value.serialize(&mut self)? as i32));
                Ok(())
            }
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut mock_serializer = MockSerializer { map: Vec::new(), error: None };

    let value = serde_json::Value::Object(
        {
            let mut map = std::collections::HashMap::new();
            map.insert("key1".to_string(), serde_json::Value::Number(1.into()));
            map.insert("key2".to_string(), serde_json::Value::Number(2.into()));
            map
        }
    );

    let result: Result<(), String> = value.serialize(mock_serializer);
    assert!(result.is_ok());
}

fn test_serialize_object_error() {
    struct MockSerializer {
        error: Option<String>,
    }

    impl serde::Serializer for MockSerializer {
        type Ok = ();
        type Error = String;

        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn serialize_map(self, _len: Option<usize>) -> Result<Self, Self::Error> {
            if self.error.is_none() {
                Ok(self)
            } else {
                Err(self.error.clone().unwrap())
            }
        }

        fn serialize_entry<K, V>(&mut self, _key: K, _value: V) -> Result<(), Self::Error>
        where
            K: serde::Serialize,
            V: serde::Serialize,
        {
            Err("serialization error".to_string())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mock_serializer = MockSerializer { error: None };

    let value = serde_json::Value::Object(
        {
            let mut map = std::collections::HashMap::new();
            map.insert("key1".to_string(), serde_json::Value::Number(1.into()));
            map
        }
    );

    let result: Result<(), String> = value.serialize(mock_serializer);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "serialization error");
}

