// Answer 0

#[derive(Debug)]
struct Content {
    name: String,
    fields: Vec<i32>,
}

impl Content {
    fn TupleStruct(name: String, fields: Vec<i32>) -> Content {
        Content { name, fields }
    }
}

struct Serializer {
    name: String,
    fields: Vec<i32>,
}

impl Serializer {
    fn end(self) -> Result<Content, &'static str> {
        Ok(Content::TupleStruct(self.name, self.fields))
    }
}

#[test]
fn test_end_with_non_empty_fields() {
    let serializer = Serializer {
        name: String::from("TestStruct"),
        fields: vec![1, 2, 3],
    };

    let result = serializer.end();
    assert!(result.is_ok());

    if let Ok(content) = result {
        assert_eq!(content.name, "TestStruct");
        assert_eq!(content.fields, vec![1, 2, 3]);
    }
}

#[test]
fn test_end_with_empty_fields() {
    let serializer = Serializer {
        name: String::from("EmptyStruct"),
        fields: vec![],
    };

    let result = serializer.end();
    assert!(result.is_ok());

    if let Ok(content) = result {
        assert_eq!(content.name, "EmptyStruct");
        assert_eq!(content.fields, vec![]);
    }
}

#[test]
fn test_end_with_long_name() {
    let serializer = Serializer {
        name: String::from("A_very_long_struct_name_that_exceeds_normal_length"),
        fields: vec![0, 1, 2],
    };

    let result = serializer.end();
    assert!(result.is_ok());

    if let Ok(content) = result {
        assert_eq!(content.name, "A_very_long_struct_name_that_exceeds_normal_length");
        assert_eq!(content.fields, vec![0, 1, 2]);
    }
}

