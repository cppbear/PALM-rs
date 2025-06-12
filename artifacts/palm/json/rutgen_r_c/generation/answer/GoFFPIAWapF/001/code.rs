// Answer 0

#[test]
fn test_from_reader_success() {
    use serde::Deserialize;
    use serde_json::Result;
    use std::io::Cursor;

    #[derive(Deserialize)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let json_data = r#"{"fingerprint": "12345", "location": "Earth"}"#;
    let cursor = Cursor::new(json_data);

    let result: Result<User> = serde_json::from_reader(cursor);
    
    assert!(result.is_ok());
    
    let user = result.unwrap();
    assert_eq!(user.fingerprint, "12345");
    assert_eq!(user.location, "Earth");
}

#[test]
fn test_from_reader_empty_input() {
    use serde::Deserialize;
    use serde_json::Result;
    use std::io::Cursor;

    #[derive(Deserialize)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let json_data = r#"{}"#;
    let cursor = Cursor::new(json_data);

    let result: Result<User> = serde_json::from_reader(cursor);
    
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_from_reader_incomplete_json() {
    use serde::Deserialize;
    use std::io::Cursor;

    #[derive(Deserialize)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let json_data = r#"{"fingerprint": "12345"}"#; // Missing location field
    let cursor = Cursor::new(json_data);

    // This should panic as there is a missing required field
    let _result: User = serde_json::from_reader(cursor).unwrap();
}

#[test]
fn test_from_reader_invalid_json() {
    use serde::Deserialize;
    use serde_json::Result;
    use std::io::Cursor;

    #[derive(Deserialize)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let json_data = r#"{fingerprint: "12345", location: "Earth"}"#; // Invalid JSON (keys must be in quotes)
    let cursor = Cursor::new(json_data);

    let result: Result<User> = serde_json::from_reader(cursor);
    
    assert!(result.is_err());
}

