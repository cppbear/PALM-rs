// Answer 0

#[test]
fn test_from_reader_success() {
    use serde::Deserialize;
    use std::io::Cursor;

    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let json_data = r#"{"fingerprint": "abc123", "location": "home"}"#;
    let reader = Cursor::new(json_data);

    let result: Result<User, _> = serde_json::from_reader(reader);
    assert!(result.is_ok());

    let user = result.unwrap();
    assert_eq!(user.fingerprint, "abc123");
    assert_eq!(user.location, "home");
}

#[test]
#[should_panic]
fn test_from_reader_invalid_json() {
    use serde::Deserialize;
    use std::io::Cursor;

    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let invalid_json_data = r#"{"fingerprint": "abc123", "location":}"#;
    let reader = Cursor::new(invalid_json_data);

    let result: Result<User, _> = serde_json::from_reader(reader);
    assert!(result.is_err());
}

#[test]
fn test_from_reader_missing_field() {
    use serde::Deserialize;
    use std::io::Cursor;

    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let json_data = r#"{"fingerprint": "abc123"}"#; // 'location' is missing
    let reader = Cursor::new(json_data);

    let result: Result<User, _> = serde_json::from_reader(reader);
    assert!(result.is_err());
}

