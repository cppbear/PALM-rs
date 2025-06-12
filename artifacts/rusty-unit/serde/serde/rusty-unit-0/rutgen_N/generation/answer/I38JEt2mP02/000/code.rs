// Answer 0

#[derive(Default)]
struct TestSerializer {
    elements: Vec<String>,
}

impl TestSerializer {
    fn new() -> Self {
        TestSerializer::default()
    }
}

impl TestSerializer {
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), &str>
    where
        T: ?Sized + serde::Serialize,
    {
        let serialized_value = serde_json::to_string(value).map_err(|_| "Serialization failed")?;
        self.elements.push(serialized_value);
        Ok(())
    }
}

#[test]
fn test_serialize_element_basic() {
    let mut serializer = TestSerializer::new();
    let value = "test";
    let result = serializer.serialize_element(&value);
    assert!(result.is_ok());
    assert_eq!(serializer.elements.len(), 1);
    assert_eq!(serializer.elements[0], "\"test\"");
}

#[test]
fn test_serialize_element_empty_string() {
    let mut serializer = TestSerializer::new();
    let value = "";
    let result = serializer.serialize_element(&value);
    assert!(result.is_ok());
    assert_eq!(serializer.elements.len(), 1);
    assert_eq!(serializer.elements[0], "\"\"");
}

#[test]
fn test_serialize_element_numeric() {
    let mut serializer = TestSerializer::new();
    let value = 42;
    let result = serializer.serialize_element(&value);
    assert!(result.is_ok());
    assert_eq!(serializer.elements.len(), 1);
    assert_eq!(serializer.elements[0], "42");
}

#[test]
fn test_serialize_element_custom_struct() {
    #[derive(serde::Serialize)]
    struct CustomStruct {
        field: String,
    }

    let mut serializer = TestSerializer::new();
    let value = CustomStruct {
        field: "value".to_string(),
    };
    let result = serializer.serialize_element(&value);
    assert!(result.is_ok());
    assert_eq!(serializer.elements.len(), 1);
    assert_eq!(serializer.elements[0], "{\"field\":\"value\"}");
}

