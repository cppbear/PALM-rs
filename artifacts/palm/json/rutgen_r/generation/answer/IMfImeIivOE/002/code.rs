// Answer 0

#[derive(Default)]
struct TestMap {
    map: std::collections::HashMap<String, serde_json::Value>,
}

impl TestMap {
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Box<dyn std::error::Error>>
    where
        T: ?Sized + serde::Serialize,
    {
        self.map.insert(String::from(key), serde_json::to_value(value)?);
        Ok(())
    }
}

#[test]
fn test_serialize_field_string() {
    let mut test_map = TestMap::default();
    let result = test_map.serialize_field("test_key", &"test_value");
    assert_eq!(result, Ok(()));
}

#[test]
fn test_serialize_field_integer() {
    let mut test_map = TestMap::default();
    let result = test_map.serialize_field("test_key", &42);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_serialize_field_float() {
    let mut test_map = TestMap::default();
    let result = test_map.serialize_field("test_key", &3.14);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_serialize_field_boolean() {
    let mut test_map = TestMap::default();
    let result = test_map.serialize_field("test_key", &true);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_serialize_field_struct() {
    #[derive(serde::Serialize)]
    struct TestStruct {
        field1: String,
        field2: i32,
    }

    let mut test_map = TestMap::default();
    let test_struct = TestStruct {
        field1: String::from("value1"),
        field2: 123,
    };
    let result = test_map.serialize_field("test_key", &test_struct);
    assert_eq!(result, Ok(()));
}

