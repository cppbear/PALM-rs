// Answer 0

#[test]
fn test_to_writer_pretty_with_valid_writer_and_empty_struct() {
    struct EmptyStruct;

    impl serde::ser::Serialize for EmptyStruct {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
        {
            serializer.serialize_unit()
        }
    }

    let writer = Vec::new();
    let value = EmptyStruct;
    to_writer_pretty(writer, &value);
}

#[test]
fn test_to_writer_pretty_with_valid_writer_and_empty_map() {
    use std::collections::HashMap;

    let writer = Vec::new();
    let value: HashMap<String, String> = HashMap::new();
    to_writer_pretty(writer, &value);
}

#[test]
fn test_to_writer_pretty_with_valid_writer_and_string_value() {
    let writer = Vec::new();
    let value = "hello world";
    to_writer_pretty(writer, &value);
}

#[test]
fn test_to_writer_pretty_with_valid_writer_and_large_vector() {
    let writer = Vec::new();
    let value = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    to_writer_pretty(writer, &value);
}

#[test]
fn test_to_writer_pretty_with_valid_writer_and_nested_structure() {
    #[derive(Serialize)]
    struct Nested {
        value: String,
        children: Vec<Nested>,
    }

    let writer = Vec::new();
    let value = Nested {
        value: "root".to_string(),
        children: vec![
            Nested {
                value: "child1".to_string(),
                children: vec![],
            },
            Nested {
                value: "child2".to_string(),
                children: vec![],
            },
        ],
    };
    to_writer_pretty(writer, &value);
}

#[test]
fn test_to_writer_pretty_with_valid_writer_and_map_with_string_keys() {
    use std::collections::HashMap;

    let writer = Vec::new();
    let mut value = HashMap::new();
    value.insert("key1".to_string(), "value1".to_string());
    value.insert("key2".to_string(), "value2".to_string());
    to_writer_pretty(writer, &value);
}

#[test]
#[should_panic]
fn test_to_writer_pretty_with_valid_writer_and_map_with_non_string_key() {
    use std::collections::HashMap;

    let writer = Vec::new();
    let value: HashMap<i32, String> = [(1, "value1".to_string())].iter().cloned().collect();
    to_writer_pretty(writer, &value);
}

