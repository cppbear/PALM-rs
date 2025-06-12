// Answer 0

#[test]
fn test_deserialize_user_success() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let json_data = r#"{
        "fingerprint": "0xF9BA143B95FF6D82",
        "location": "Menlo Park, CA"
    }"#;

    let user: User = serde_json::from_str(json_data).unwrap();
    
    assert_eq!(user.fingerprint, "0xF9BA143B95FF6D82");
    assert_eq!(user.location, "Menlo Park, CA");
}

#[test]
fn test_deserialize_user_missing_field() {
    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let json_data = r#"{
        "fingerprint": "0xF9BA143B95FF6D82"
    }"#;

    let result: Result<User, serde_json::Error> = serde_json::from_str(json_data);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_user_invalid_json() {
    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let json_data = r#"
    {
        "fingerprint": "0xF9BA143B95FF6D82",
        "location": "Menlo Park, CA",
    }"#; // Extra comma should trigger an error

    let result: Result<User, serde_json::Error> = serde_json::from_str(json_data);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_user_empty_json() {
    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let json_data = r#"{}"#;

    let result: Result<User, serde_json::Error> = serde_json::from_str(json_data);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_user_wrong_type() {
    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let json_data = r#""Not a user object""#; // String instead of JSON object

    let result: Result<User, serde_json::Error> = serde_json::from_str(json_data);
    assert!(result.is_err());
}

