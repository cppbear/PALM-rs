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
        let value = serde_json::to_string(value).map_err(|e| e.to_string())?;
        self.fields.push(value);
        Ok(())
    }
}

#[test]
fn test_serialize_field_with_string() {
    let mut serializer = TestSerializer::new();
    let value = "test string";
    assert_eq!(serializer.serialize_field(&value), Ok(()));
    assert_eq!(serializer.fields, vec![r#""test string""#.to_string()]);
}

#[test]
fn test_serialize_field_with_integer() {
    let mut serializer = TestSerializer::new();
    let value = 42;
    assert_eq!(serializer.serialize_field(&value), Ok(()));
    assert_eq!(serializer.fields, vec![r#"42"#.to_string()]);
}

#[test]
fn test_serialize_field_with_struct() {
    #[derive(serde::Serialize)]
    struct TestStruct {
        id: u32,
        name: String,
    }

    let mut serializer = TestSerializer::new();
    let value = TestStruct {
        id: 1,
        name: "test".to_string(),
    };
    assert_eq!(serializer.serialize_field(&value), Ok(()));
    assert_eq!(serializer.fields, vec![r#"{"id":1,"name":"test"}"#.to_string()]);
}

#[test]
fn test_serialize_field_with_empty_string() {
    let mut serializer = TestSerializer::new();
    let value = "";
    assert_eq!(serializer.serialize_field(&value), Ok(()));
    assert_eq!(serializer.fields, vec![r#""""#.to_string()]);
}

