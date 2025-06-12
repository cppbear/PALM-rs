// Answer 0

#[derive(Debug)]
struct Content {
    name: String,
    fields: Vec<String>,
}

struct StructSerializer {
    name: String,
    fields: Vec<String>,
}

impl StructSerializer {
    fn new(name: &str) -> Self {
        Self { 
            name: name.to_string(), 
            fields: Vec::new() 
        }
    }
    
    fn add_field(&mut self, field: &str) {
        self.fields.push(field.to_string());
    }

    fn end(self) -> Result<Content, &'static str> {
        Ok(Content::Struct(self.name, self.fields))
    }
}

#[test]
fn test_end_empty_fields() {
    let serializer = StructSerializer::new("TestStruct");
    let result = serializer.end().unwrap();
    assert_eq!(result.name, "TestStruct");
    assert!(result.fields.is_empty());
}

#[test]
fn test_end_with_fields() {
    let mut serializer = StructSerializer::new("TestStruct");
    serializer.add_field("field1");
    serializer.add_field("field2");
    let result = serializer.end().unwrap();
    assert_eq!(result.name, "TestStruct");
    assert_eq!(result.fields.len(), 2);
    assert_eq!(result.fields[0], "field1");
    assert_eq!(result.fields[1], "field2");
}

#[test]
#[should_panic]
fn test_end_panic_on_invalid_state() {
    // Implement a scenario that may lead to an invalid state if applicable 
    // For example, we can force an incorrect state manipulation before calling end
    let serializer = StructSerializer::new("TestStruct");
    let _ = serializer.end(); // If there were a requirement that fields cannot be empty
}

