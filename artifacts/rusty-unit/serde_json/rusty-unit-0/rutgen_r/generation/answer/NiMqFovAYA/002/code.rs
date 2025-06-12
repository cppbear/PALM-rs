// Answer 0

#[derive(Debug)]
struct Serializer {
    vec: Vec<serde_json::Value>,
}

impl Serializer {
    fn new() -> Self {
        Serializer { vec: Vec::new() }
    }

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        self.vec.push(serde_json::to_value(value)?);
        Ok(())
    }
}

#[test]
fn test_serialize_field_with_integer() {
    let mut serializer = Serializer::new();
    let value = 42;

    let result = serializer.serialize_field(&value);
    assert!(result.is_ok());
    assert_eq!(serializer.vec.len(), 1);
    assert_eq!(serializer.vec[0], serde_json::Value::from(42));
}

#[test]
fn test_serialize_field_with_string() {
    let mut serializer = Serializer::new();
    let value = "Hello, world!";

    let result = serializer.serialize_field(&value);
    assert!(result.is_ok());
    assert_eq!(serializer.vec.len(), 1);
    assert_eq!(serializer.vec[0], serde_json::Value::from("Hello, world!"));
}

#[test]
fn test_serialize_field_with_struct() {
    #[derive(serde::Serialize)]
    struct SampleStruct {
        name: String,
        value: i32,
    }

    let mut serializer = Serializer::new();
    let value = SampleStruct {
        name: String::from("Test"),
        value: 123,
    };

    let result = serializer.serialize_field(&value);
    assert!(result.is_ok());
    assert_eq!(serializer.vec.len(), 1);
    assert_eq!(
        serializer.vec[0],
        serde_json::json!({"name": "Test", "value": 123})
    );
}

#[test]
fn test_serialize_field_with_empty_vector() {
    let mut serializer = Serializer::new();
    let value: Vec<i32> = Vec::new();

    let result = serializer.serialize_field(&value);
    assert!(result.is_ok());
    assert_eq!(serializer.vec.len(), 1);
    assert_eq!(serializer.vec[0], serde_json::Value::Array(vec![]));
}

