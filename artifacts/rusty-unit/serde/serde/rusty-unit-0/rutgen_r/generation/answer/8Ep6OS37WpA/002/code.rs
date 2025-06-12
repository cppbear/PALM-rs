// Answer 0

#[derive(Serialize)]
struct TestValue {
    data: u32,
}

struct ContentSerializer<E> {
    // Assume there's some fields needed in the serializer
}

impl<E> ContentSerializer<E> {
    pub fn new() -> Self {
        ContentSerializer {
            // Initialize fields if necessary
        }
    }
}

struct TestStruct {
    fields: Vec<(&'static str, String)>,
}

impl TestStruct {
    pub fn new() -> Self {
        TestStruct {
            fields: Vec::new(),
        }
    }

    pub fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), String>
    where
        T: ?Sized + Serialize,
    {
        let value = value.serialize(ContentSerializer::<String>::new()).map_err(|_| "Serialize error".to_string())?;
        self.fields.push((key, value));
        Ok(())
    }
}

#[test]
fn test_serialize_field_success() {
    let mut test_struct = TestStruct::new();
    let value = TestValue { data: 42 };
    let result = test_struct.serialize_field("test_key", &value);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_serialize_field_empty_key() {
    let mut test_struct = TestStruct::new();
    let value = TestValue { data: 100 };
    let result = test_struct.serialize_field("", &value);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_serialize_field_multiple_calls() {
    let mut test_struct = TestStruct::new();
    let value1 = TestValue { data: 1 };
    let value2 = TestValue { data: 2 };
    let result1 = test_struct.serialize_field("key1", &value1);
    let result2 = test_struct.serialize_field("key2", &value2);
    assert_eq!(result1, Ok(()));
    assert_eq!(result2, Ok(()));
}

