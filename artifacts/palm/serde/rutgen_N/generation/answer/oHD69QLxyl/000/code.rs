// Answer 0

#[derive(Debug)]
struct Content {
    name: String,
    fields: Vec<String>,
}

impl Content {
    fn tuple_struct(name: String, fields: Vec<String>) -> Content {
        Content { name, fields }
    }
}

#[derive(Debug)]
struct SerdeTest {
    name: String,
    fields: Vec<String>,
}

impl SerdeTest {
    fn new(name: &str, fields: Vec<String>) -> Self {
        SerdeTest {
            name: name.to_string(),
            fields,
        }
    }

    fn end(self) -> Result<Content, String> {
        Ok(Content::tuple_struct(self.name, self.fields))
    }
}

#[test]
fn test_end_with_valid_data() {
    let test_instance = SerdeTest::new("TestName", vec!["field1".into(), "field2".into()]);
    let result = test_instance.end();
    assert!(result.is_ok());
    if let Ok(content) = result {
        assert_eq!(content.name, "TestName");
        assert_eq!(content.fields.len(), 2);
        assert_eq!(content.fields[0], "field1");
        assert_eq!(content.fields[1], "field2");
    }
}

#[test]
fn test_end_with_empty_fields() {
    let test_instance = SerdeTest::new("EmptyFields", vec![]);
    let result = test_instance.end();
    assert!(result.is_ok());
    if let Ok(content) = result {
        assert_eq!(content.name, "EmptyFields");
        assert_eq!(content.fields.len(), 0);
    }
}

