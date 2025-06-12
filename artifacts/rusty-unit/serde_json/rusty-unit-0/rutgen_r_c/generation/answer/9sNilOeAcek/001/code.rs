// Answer 0

#[test]
fn test_to_writer_pretty_string() {
    use std::io::Cursor;
    use serde_json::json;

    let value = json!({
        "name": "John Doe",
        "age": 30,
        "is_student": false,
    });

    let mut output = Vec::new();
    let writer = Cursor::new(&mut output);
    
    let result = to_writer_pretty(writer, &value);
    
    assert!(result.is_ok());
    let expected_output = r#"{
  "name": "John Doe",
  "age": 30,
  "is_student": false
}"#;
    assert_eq!(String::from_utf8(output).unwrap(), expected_output);
}

#[test]
#[should_panic]
fn test_to_writer_pretty_map_with_non_string_key() {
    use std::collections::HashMap;
    use std::io::Cursor;

    let mut map: HashMap<i32, String> = HashMap::new();
    map.insert(1, "value".to_string());

    let writer = Cursor::new(Vec::new());

    let _ = to_writer_pretty(writer, &map); // This should panic
}

#[test]
fn test_to_writer_pretty_empty_structure() {
    use std::io::Cursor;
    use serde_json::json;

    let value = json!({});

    let mut output = Vec::new();
    let writer = Cursor::new(&mut output);
    
    let result = to_writer_pretty(writer, &value);
    
    assert!(result.is_ok());
    let expected_output = r#"{}"#;
    assert_eq!(String::from_utf8(output).unwrap(), expected_output);
}

#[test]
fn test_to_writer_pretty_nested_structures() {
    use std::io::Cursor;
    use serde_json::json;

    let value = json!({
        "user": {
            "name": "Alice",
            "preferences": {
                "color": "blue",
                "food": "pizza",
            }
        }
    });

    let mut output = Vec::new();
    let writer = Cursor::new(&mut output);
    
    let result = to_writer_pretty(writer, &value);
    
    assert!(result.is_ok());
    let expected_output = r#"{
  "user": {
    "name": "Alice",
    "preferences": {
      "color": "blue",
      "food": "pizza"
    }
  }
}"#;
    assert_eq!(String::from_utf8(output).unwrap(), expected_output);
}

