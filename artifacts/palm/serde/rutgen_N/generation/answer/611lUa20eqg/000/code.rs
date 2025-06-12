// Answer 0

#[derive(Default)]
struct TestSerializer {
    fields: Vec<String>,
}

impl TestSerializer {
    fn new() -> Self {
        Self::default()
    }

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), String>
    where
        T: ?Sized + serde::Serialize,
    {
        let serialized_value = value.serialize(ContentSerializer::new()).map_err(|_| "Serialization Error".to_string())?;
        self.fields.push(serialized_value);
        Ok(())
    }
}

struct ContentSerializer;

impl ContentSerializer {
    fn new() -> Self {
        ContentSerializer
    }
}

impl serde::Serializer for ContentSerializer {
    type Ok = String;
    type Error = String;

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    // Other required serializer methods would be here, but kept minimal for the test.
}

#[derive(serde::Serialize)]
struct TestStruct {
    field: String,
}

#[test]
fn test_serialize_field() {
    let mut serializer = TestSerializer::new();
    let test_value = TestStruct {
        field: "test".to_string(),
    };
    
    let result = serializer.serialize_field(&test_value);
    assert!(result.is_ok());
    assert_eq!(serializer.fields.len(), 1);
    assert_eq!(serializer.fields[0], "test");
}

#[test]
fn test_serialize_field_empty_string() {
    let mut serializer = TestSerializer::new();
    let test_value = TestStruct {
        field: "".to_string(),
    };
    
    let result = serializer.serialize_field(&test_value);
    assert!(result.is_ok());
    assert_eq!(serializer.fields.len(), 1);
    assert_eq!(serializer.fields[0], "");
}

