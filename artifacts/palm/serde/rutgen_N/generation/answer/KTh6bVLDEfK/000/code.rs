// Answer 0

#[derive(Debug)]
struct Content {
    name: String,
    fields: Vec<String>,
}

struct Serializer {
    name: String,
    fields: Vec<String>,
}

impl Serializer {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            fields: Vec::new(),
        }
    }

    fn end(self) -> Result<Content, &'static str> {
        Ok(Content {
            name: self.name,
            fields: self.fields,
        })
    }
}

#[test]
fn test_end_function_with_valid_data() {
    let serializer = Serializer::new("TestStruct");
    let result = serializer.end();
    
    assert!(result.is_ok());
    let content = result.unwrap();
    assert_eq!(content.name, "TestStruct");
    assert!(content.fields.is_empty());
}

#[test]
fn test_end_function_with_fields() {
    let mut serializer = Serializer::new("TestStruct");
    serializer.fields.push("field1".to_string());
    serializer.fields.push("field2".to_string());
    
    let result = serializer.end();
    
    assert!(result.is_ok());
    let content = result.unwrap();
    assert_eq!(content.name, "TestStruct");
    assert_eq!(content.fields.len(), 2);
    assert_eq!(content.fields[0], "field1");
    assert_eq!(content.fields[1], "field2");
}

