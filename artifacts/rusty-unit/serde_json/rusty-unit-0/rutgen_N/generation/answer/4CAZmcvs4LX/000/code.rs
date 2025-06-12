// Answer 0

#[derive(Default)]
struct TestSerializer {
    vec: Vec<serde_json::Value>,
}

impl TestSerializer {
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        self.vec.push(serde_json::to_value(value)?);
        Ok(())
    }
}

#[test]
fn test_serialize_element_with_integer() {
    let mut serializer = TestSerializer::default();
    let value = 42;

    let result = serializer.serialize_element(&value);
    
    assert!(result.is_ok());
    assert_eq!(serializer.vec.len(), 1);
    assert_eq!(serializer.vec[0], serde_json::to_value(42).unwrap());
}

#[test]
fn test_serialize_element_with_string() {
    let mut serializer = TestSerializer::default();
    let value = "Hello, world!";

    let result = serializer.serialize_element(&value);
    
    assert!(result.is_ok());
    assert_eq!(serializer.vec.len(), 1);
    assert_eq!(serializer.vec[0], serde_json::to_value("Hello, world!").unwrap());
}

#[test]
fn test_serialize_element_with_struct() {
    #[derive(serde::Serialize)]
    struct TestStruct {
        x: i32,
        y: String,
    }

    let mut serializer = TestSerializer::default();
    let value = TestStruct { x: 10, y: "Test".to_string() };

    let result = serializer.serialize_element(&value);
    
    assert!(result.is_ok());
    assert_eq!(serializer.vec.len(), 1);
    assert_eq!(serializer.vec[0], serde_json::to_value(&value).unwrap());
}

