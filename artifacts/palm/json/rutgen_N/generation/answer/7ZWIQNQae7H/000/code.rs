// Answer 0

#[test]
fn test_from_reader_with_valid_json() {
    use serde_json::Deserializer;
    use std::io::Cursor;

    let data = r#"{"key": "value"}"#;
    let reader = Cursor::new(data);

    let deserializer = Deserializer::from_reader(reader);
    let result: serde_json::Value = serde_json::from_reader(deserializer).unwrap();

    assert_eq!(result["key"], "value");
}

#[test]
#[should_panic]
fn test_from_reader_with_invalid_json() {
    use serde_json::Deserializer;
    use std::io::Cursor;

    let invalid_data = r#"{key: value}"#; // Invalid JSON format
    let reader = Cursor::new(invalid_data);

    let deserializer = Deserializer::from_reader(reader);
    let _: serde_json::Value = serde_json::from_reader(deserializer).unwrap();
}

