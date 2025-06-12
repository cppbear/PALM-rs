// Answer 0

#[test]
fn test_from_str_success() {
    #[derive(serde::Deserialize, Debug, PartialEq)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let json_data = r#"
    {
        "fingerprint": "0xF9BA143B95FF6D82",
        "location": "Menlo Park, CA"
    }
    "#;

    let user: User = serde_json::from_str(json_data).unwrap();
    assert_eq!(user.fingerprint, "0xF9BA143B95FF6D82");
    assert_eq!(user.location, "Menlo Park, CA");
}

#[test]
fn test_from_str_missing_field() {
    #[derive(serde::Deserialize, Debug, PartialEq)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let json_data = r#"
    {
        "fingerprint": "0xF9BA143B95FF6D82"
    }
    "#;

    let result: Result<User, _> = serde_json::from_str(json_data);
    assert!(result.is_err());
}

#[test]
fn test_from_str_invalid_json() {
    #[derive(serde::Deserialize, Debug, PartialEq)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let json_data = r#"
    {
        "fingerprint": "0xF9BA143B95FF6D82",
        "location": "Menlo Park, CA"
    "#;

    let result: Result<User, _> = serde_json::from_str(json_data);
    assert!(result.is_err());
}

#[test]
fn test_from_str_type_mismatch() {
    #[derive(serde::Deserialize, Debug, PartialEq)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let json_data = r#"
    {
        "fingerprint": "0xF9BA143B95FF6D82",
        "location": 42
    }
    "#;

    let result: Result<User, _> = serde_json::from_str(json_data);
    assert!(result.is_err());
}

#[test]
fn test_from_str_empty_string() {
    #[derive(serde::Deserialize, Debug, PartialEq)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let json_data = r#""#;

    let result: Result<User, _> = serde_json::from_str(json_data);
    assert!(result.is_err());
}

