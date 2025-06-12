// Answer 0

#[derive(Default)]
struct TestSerializer {
    key: Option<String>,
    entries: Vec<(String, String)>,
}

impl TestSerializer {
    fn new() -> Self {
        Self::default()
    }

    fn serialize_key(&mut self, key: String) {
        self.key = Some(key);
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), String>
    where
        T: ?Sized + serde::Serialize,
    {
        let key = self
            .key
            .take()
            .expect("serialize_value called before serialize_key");
        let value = serde_json::to_string(value).map_err(|e| e.to_string())?;
        self.entries.push((key, value));
        Ok(())
    }
}

#[test]
fn test_serialize_value() {
    let mut serializer = TestSerializer::new();
    serializer.serialize_key("test_key".to_string()).unwrap();
    
    let result = serializer.serialize_value(&"test_value");
    
    assert!(result.is_ok());
    assert_eq!(serializer.entries.len(), 1);
    assert_eq!(serializer.entries[0].0, "test_key");
    assert_eq!(serializer.entries[0].1, "\"test_value\"");
}

#[test]
fn test_serialize_value_without_key() {
    let mut serializer = TestSerializer::new();
    
    let result = serializer.serialize_value(&"test_value");
    
    assert!(result.is_err());
}

#[test]
fn test_serialize_value_with_struct() {
    #[derive(serde::Serialize)]
    struct TestStruct {
        id: u32,
        name: String,
    }

    let mut serializer = TestSerializer::new();
    serializer.serialize_key("struct_key".to_string()).unwrap();
    
    let test_struct = TestStruct { id: 1, name: "Example".to_string() };
    let result = serializer.serialize_value(&test_struct);
    
    assert!(result.is_ok());
    assert_eq!(serializer.entries.len(), 1);
    assert_eq!(serializer.entries[0].0, "struct_key");
    assert_eq!(serializer.entries[0].1, "{\"id\":1,\"name\":\"Example\"}");
}

