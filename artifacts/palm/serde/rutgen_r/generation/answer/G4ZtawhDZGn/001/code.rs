// Answer 0

#[derive(Debug)]
struct Content {
    name: String,
    variant_index: usize,
    variant: String,
    fields: Vec<String>,
}

#[derive(Debug)]
struct TestStruct {
    name: String,
    variant_index: usize,
    variant: String,
    fields: Vec<String>,
}

impl TestStruct {
    fn end(self) -> Result<Content, &'static str> {
        Ok(Content {
            name: self.name,
            variant_index: self.variant_index,
            variant: self.variant,
            fields: self.fields,
        })
    }
}

#[test]
fn test_end_success() {
    let test_instance = TestStruct {
        name: "TestName".to_string(),
        variant_index: 0,
        variant: "TestVariant".to_string(),
        fields: vec!["field1".to_string(), "field2".to_string()],
    };

    let result = test_instance.end();

    match result {
        Ok(content) => {
            assert_eq!(content.name, "TestName");
            assert_eq!(content.variant_index, 0);
            assert_eq!(content.variant, "TestVariant");
            assert_eq!(content.fields, vec!["field1".to_string(), "field2".to_string()]);
        }
        Err(_) => panic!("Expected Ok but got Err"),
    }
}

#[test]
fn test_end_empty_fields() {
    let test_instance = TestStruct {
        name: "EmptyFields".to_string(),
        variant_index: 1,
        variant: "EmptyVariant".to_string(),
        fields: vec![],
    };

    let result = test_instance.end();

    match result {
        Ok(content) => {
            assert_eq!(content.name, "EmptyFields");
            assert_eq!(content.variant_index, 1);
            assert_eq!(content.variant, "EmptyVariant");
            assert!(content.fields.is_empty());
        }
        Err(_) => panic!("Expected Ok but got Err"),
    }
}

#[test]
fn test_end_negative_variant_index() {
    let test_instance = TestStruct {
        name: "NegativeIndex".to_string(),
        variant_index: usize::MAX, // This can stand for an invalid index in typical contexts
        variant: "InvalidVariant".to_string(),
        fields: vec!["field".to_string()],
    };

    let result = test_instance.end();

    match result {
        Ok(content) => {
            assert_eq!(content.name, "NegativeIndex");
            assert_eq!(content.variant_index, usize::MAX);
            assert_eq!(content.variant, "InvalidVariant");
            assert_eq!(content.fields, vec!["field".to_string()]);
        }
        Err(_) => panic!("Expected Ok but got Err"),
    }
}

