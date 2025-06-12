// Answer 0

#[test]
fn test_to_writer_pretty_valid_data() {
    use serde_json::json;
    use std::io::Cursor;
    
    let value = json!({
        "name": "Alice",
        "age": 30,
        "is_student": false,
        "courses": ["Math", "Science"],
    });
    
    let mut buffer = Vec::new();
    let result = serde_json::to_writer_pretty(&mut buffer, &value);
    assert!(result.is_ok());
    
    let expected_output = r#"{
  "name": "Alice",
  "age": 30,
  "is_student": false,
  "courses": [
    "Math",
    "Science"
  ]
}"#;
    assert_eq!(String::from_utf8(buffer).unwrap(), expected_output);
}

#[test]
#[should_panic]
fn test_to_writer_pretty_invalid_key_in_map() {
    use serde_json::json;
    use std::io::Cursor;

    struct NonStringKey { key: i32 }
    
    let value = json!({
        NonStringKey { key: 1 }: "value"
    });

    let mut buffer = Vec::new();
    let _ = serde_json::to_writer_pretty(&mut buffer, &value);
}

#[test]
fn test_to_writer_pretty_empty_struct() {
    use serde::Serialize;

    #[derive(Serialize)]
    struct Empty;

    let value = Empty;
    
    let mut buffer = Vec::new();
    let result = serde_json::to_writer_pretty(&mut buffer, &value);
    assert!(result.is_ok());
    
    let expected_output = "{}\n";
    assert_eq!(String::from_utf8(buffer).unwrap(), expected_output);
}

#[test]
fn test_to_writer_pretty_special_characters() {
    use serde_json::json;
    use std::io::Cursor;

    let value = json!({
        "text": "This is a test: \n\t\u{2602}",
    });

    let mut buffer = Vec::new();
    let result = serde_json::to_writer_pretty(&mut buffer, &value);
    assert!(result.is_ok());
    
    let expected_output = r#"{
  "text": "This is a test: 
	☂"
}"#;
    assert_eq!(String::from_utf8(buffer).unwrap(), expected_output);
}

