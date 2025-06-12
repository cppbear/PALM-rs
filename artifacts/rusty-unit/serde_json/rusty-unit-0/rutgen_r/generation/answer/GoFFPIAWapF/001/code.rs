// Answer 0

#[test]
fn test_deserialize_user_success() {
    use serde::Deserialize;
    use std::io::Cursor;

    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let json_data = r#"{ "fingerprint": "abc123", "location": "Nowhere" }"#;
    let cursor = Cursor::new(json_data);

    let result: Result<User, _> = serde_json::from_reader(cursor);
    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user.fingerprint, "abc123");
    assert_eq!(user.location, "Nowhere");
}

#[test]
#[should_panic]
fn test_deserialize_user_missing_field() {
    use serde::Deserialize;
    use std::io::Cursor;

    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let json_data = r#"{ "location": "Nowhere" }"#; // Missing fingerprint
    let cursor = Cursor::new(json_data);

    let _: Result<User, _> = serde_json::from_reader(cursor).unwrap(); // This should panic
}

#[test]
#[should_panic]
fn test_deserialize_user_invalid_json() {
    use serde::Deserialize;
    use std::io::Cursor;

    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let json_data = r#"{ "fingerprint": "abc123", "location": "Nowhere" "#; // Invalid JSON
    let cursor = Cursor::new(json_data);

    let _: Result<User, _> = serde_json::from_reader(cursor).unwrap(); // This should panic
}

