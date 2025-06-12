// Answer 0

#[derive(Debug)]
struct Content {
    name: String,
    variant_index: usize,
    variant: String,
    fields: Vec<String>,
}

#[derive(Debug)]
struct Serializer {
    name: String,
    variant_index: usize,
    variant: String,
    fields: Vec<String>,
}

impl Serializer {
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
    let serializer = Serializer {
        name: "Example".to_string(),
        variant_index: 0,
        variant: "VariantA".to_string(),
        fields: vec!["field1".to_string(), "field2".to_string()],
    };

    let result = serializer.end().unwrap();

    assert_eq!(result.name, "Example");
    assert_eq!(result.variant_index, 0);
    assert_eq!(result.variant, "VariantA");
    assert_eq!(result.fields, vec!["field1".to_string(), "field2".to_string()]);
}

#[test]
fn test_end_empty_fields() {
    let serializer = Serializer {
        name: "Example".to_string(),
        variant_index: 1,
        variant: "VariantB".to_string(),
        fields: vec![],
    };

    let result = serializer.end().unwrap();

    assert_eq!(result.name, "Example");
    assert_eq!(result.variant_index, 1);
    assert_eq!(result.variant, "VariantB");
    assert!(result.fields.is_empty());
}

