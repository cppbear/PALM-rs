// Answer 0

#[derive(Debug)]
struct Content {
    name: String,
    variant_index: usize,
    variant: String,
    fields: Vec<String>,
}

impl Content {
    fn StructVariant(name: String, variant_index: usize, variant: String, fields: Vec<String>) -> Self {
        Content {
            name,
            variant_index,
            variant,
            fields,
        }
    }
}

struct TestStruct {
    name: String,
    variant_index: usize,
    variant: String,
    fields: Vec<String>,
}

impl TestStruct {
    fn end(self) -> Result<Content, &'static str> {
        Ok(Content::StructVariant(
            self.name,
            self.variant_index,
            self.variant,
            self.fields,
        ))
    }
}

#[test]
fn test_end_with_valid_data() {
    let test_instance = TestStruct {
        name: String::from("TestStruct"),
        variant_index: 1,
        variant: String::from("VariantA"),
        fields: vec![String::from("field1"), String::from("field2")],
    };

    let result = test_instance.end();
    assert_eq!(result.is_ok(), true);
    
    if let Ok(content) = result {
        assert_eq!(content.name, "TestStruct");
        assert_eq!(content.variant_index, 1);
        assert_eq!(content.variant, "VariantA");
        assert_eq!(content.fields, vec!["field1", "field2"]);
    }
}

#[test]
fn test_end_with_empty_fields() {
    let test_instance = TestStruct {
        name: String::from("EmptyFieldsStruct"),
        variant_index: 0,
        variant: String::from("VariantB"),
        fields: vec![],
    };

    let result = test_instance.end();
    assert_eq!(result.is_ok(), true);
    
    if let Ok(content) = result {
        assert_eq!(content.name, "EmptyFieldsStruct");
        assert_eq!(content.variant_index, 0);
        assert_eq!(content.variant, "VariantB");
        assert_eq!(content.fields.len(), 0);
    }
}

