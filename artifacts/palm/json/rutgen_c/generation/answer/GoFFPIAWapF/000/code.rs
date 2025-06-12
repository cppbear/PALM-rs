// Answer 0

#[test]
fn test_from_reader_valid_json() {
    use serde::Deserialize;
    use std::io::Cursor;

    #[derive(Deserialize, Debug, PartialEq)]
    struct TestData {
        name: String,
        age: usize,
    }

    let json_data = r#"{"name": "Alice", "age": 30}"#;
    let cursor = Cursor::new(json_data);

    let result: TestData = from_reader(cursor).unwrap();
    assert_eq!(result, TestData { name: "Alice".to_string(), age: 30 });
}

#[test]
fn test_from_reader_invalid_json() {
    use serde::Deserialize;
    use std::io::Cursor;

    #[derive(Deserialize, Debug)]
    struct TestData {
        name: String,
        age: usize,
    }

    let json_data = r#"{"name": "Alice", "age": "thirty"}"#;
    let cursor = Cursor::new(json_data);

    let result: Result<TestData> = from_reader(cursor);
    assert!(result.is_err());
}

#[test]
fn test_from_reader_incomplete_json() {
    use serde::Deserialize;
    use std::io::Cursor;

    #[derive(Deserialize, Debug)]
    struct TestData {
        name: String,
        age: usize,
    }

    let json_data = r#"{"name": "Alice""#; // Incomplete JSON
    let cursor = Cursor::new(json_data);

    let result: Result<TestData> = from_reader(cursor);
    assert!(result.is_err());
}

#[test]
fn test_from_reader_empty_input() {
    use serde::Deserialize;
    use std::io::Cursor;

    #[derive(Deserialize, Debug)]
    struct TestData {
        name: String,
        age: usize,
    }

    let json_data = r#"{}"#; // No required fields
    let cursor = Cursor::new(json_data);

    let result: Result<TestData> = from_reader(cursor);
    assert!(result.is_err());
}

