// Answer 0

#[derive(Default)]
struct MockSerializer;

impl MockSerializer {
    fn serialize_value(&self, _value: &Content) -> Result<(), String> {
        Ok(())
    }
    
    fn end(&self) -> Result<(), String> {
        Ok(())
    }
}

struct Content {
    name: String,
    fields: Vec<String>,
}

impl Content {
    fn struct_new(name: String, fields: Vec<String>) -> Self {
        Content { name, fields }
    }
}

struct StructWithMap {
    name: String,
    fields: Vec<String>,
    map: MockSerializer,
}

impl StructWithMap {
    fn end(mut self) -> Result<(), String> {
        self.map.serialize_value(&Content::struct_new(self.name, self.fields))?;
        self.map.end()
    }
}

#[test]
fn test_end_success() {
    let struct_with_map = StructWithMap {
        name: String::from("test"),
        fields: vec![String::from("field1"), String::from("field2")],
        map: MockSerializer::default(),
    };
    
    let result = struct_with_map.end();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_end_failure_on_serialize() {
    struct FailingSerializer;

    impl FailingSerializer {
        fn serialize_value(&self, _value: &Content) -> Result<(), String> {
            Err(String::from("Serialization failed"))
        }

        fn end(&self) -> Result<(), String> {
            Ok(())
        }
    }

    let struct_with_map = StructWithMap {
        name: String::from("test"),
        fields: vec![String::from("field1"), String::from("field2")],
        map: FailingSerializer,
    };

    let result = struct_with_map.end();
    assert!(result.is_err());
}

