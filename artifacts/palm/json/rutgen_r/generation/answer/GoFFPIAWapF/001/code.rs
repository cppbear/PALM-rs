// Answer 0

#[test]
fn test_from_reader_valid_json() {
    use serde::Deserialize;
    use serde_json::Value;
    use std::io::Cursor;

    #[derive(Deserialize, Debug, PartialEq)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let json_data = r#"{"fingerprint": "ABC123", "location": "Earth"}"#;
    let reader = Cursor::new(json_data);

    let user: User = serde_json::from_reader(reader).unwrap();
    assert_eq!(user, User { fingerprint: String::from("ABC123"), location: String::from("Earth") });
}

#[test]
#[should_panic(expected = "expected ident")]
fn test_from_reader_invalid_json() {
    use serde::Deserialize;
    use std::io::Cursor;

    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let invalid_json_data = r#"{"fingerprint": "ABC123", "location": "Earth""#; // Missing closing brace
    let reader = Cursor::new(invalid_json_data);

    let _: User = serde_json::from_reader(reader).unwrap(); // This should panic
}

#[test]
fn test_from_reader_missing_field() {
    use serde::Deserialize;
    use std::io::Cursor;

    #[derive(Deserialize, Debug, PartialEq)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let json_data = r#"{"fingerprint": "ABC123"}"#; // Missing 'location'
    let reader = Cursor::new(json_data);

    let result: Result<User, _> = serde_json::from_reader(reader);
    assert!(result.is_err());
}

#[test]
fn test_from_reader_empty_json() {
    use serde::Deserialize;
    use std::io::Cursor;

    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let empty_json = r#"{}"#;
    let reader = Cursor::new(empty_json);

    let result: Result<User, _> = serde_json::from_reader(reader);
    assert!(result.is_err());
}

#[test]
fn test_from_reader_mismatched_types() {
    use serde::Deserialize;
    use std::io::Cursor;

    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
        age: u32,
    }

    let json_data = r#"{"fingerprint": "ABC123", "location": "Earth", "age": "twenty"}"#; // age is a string, should be a number
    let reader = Cursor::new(json_data);

    let result: Result<User, _> = serde_json::from_reader(reader);
    assert!(result.is_err());
}

