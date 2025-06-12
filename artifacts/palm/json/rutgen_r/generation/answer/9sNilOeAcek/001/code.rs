// Answer 0

#[test]
fn test_to_writer_pretty_with_valid_json() {
    use std::io::Cursor;
    use serde::Serialize;
    use serde_json::to_writer_pretty;

    #[derive(Serialize)]
    struct ValidData {
        name: String,
        age: u32,
    }

    let data = ValidData {
        name: "Alice".to_string(),
        age: 30,
    };

    let mut buffer = Cursor::new(Vec::new());
    let result = to_writer_pretty(&mut buffer, &data);
    assert!(result.is_ok());

    let output = buffer.into_inner();
    let expected = r#"{ "name": "Alice", "age": 30 }"#;
    assert_eq!(String::from_utf8_lossy(&output), expected);
}

#[test]
#[should_panic]
fn test_to_writer_pretty_with_non_string_key() {
    use std::io::Cursor;
    use serde::Serialize;
    use serde_json::to_writer_pretty;
    use std::collections::HashMap;

    #[derive(Serialize)]
    struct InvalidData {
        items: HashMap<u32, String>,
    }

    let mut map = HashMap::new();
    map.insert(1, "One".to_string());

    let data = InvalidData { items: map };

    let mut buffer = Cursor::new(Vec::new());
    let _ = to_writer_pretty(&mut buffer, &data);
}

#[test]
fn test_to_writer_pretty_with_empty_struct() {
    use std::io::Cursor;
    use serde::Serialize;
    use serde_json::to_writer_pretty;

    #[derive(Serialize)]
    struct EmptyData;

    let data = EmptyData;

    let mut buffer = Cursor::new(Vec::new());
    let result = to_writer_pretty(&mut buffer, &data);
    assert!(result.is_ok());

    let output = buffer.into_inner();
    let expected = r#"{}"#;
    assert_eq!(String::from_utf8_lossy(&output), expected);
}

#[test]
fn test_to_writer_pretty_with_nested_structs() {
    use std::io::Cursor;
    use serde::Serialize;
    use serde_json::to_writer_pretty;

    #[derive(Serialize)]
    struct Child {
        name: String,
    }

    #[derive(Serialize)]
    struct Parent {
        child: Child,
    }

    let data = Parent {
        child: Child {
            name: "Bob".to_string(),
        },
    };

    let mut buffer = Cursor::new(Vec::new());
    let result = to_writer_pretty(&mut buffer, &data);
    assert!(result.is_ok());

    let output = buffer.into_inner();
    let expected = r#"{
  "child": {
    "name": "Bob"
  }
}"#;
    assert_eq!(String::from_utf8_lossy(&output), expected);
}

