// Answer 0

#[test]
fn test_deserialize_valid_user() {
    use serde::Deserialize;

    #[derive(Deserialize, Debug, PartialEq)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let json_data = r#"
    {
        "fingerprint": "0xF9BA143B95FF6D82",
        "location": "Menlo Park, CA"
    }"#;

    let user: User = serde_json::from_str(json_data).unwrap();
    assert_eq!(user.fingerprint, "0xF9BA143B95FF6D82");
    assert_eq!(user.location, "Menlo Park, CA");
}

#[test]
#[should_panic(expected = "missing field")]
fn test_deserialize_missing_field() {
    use serde::Deserialize;

    #[derive(Deserialize, Debug, PartialEq)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let json_data = r#"
    {
        "fingerprint": "0xF9BA143B95FF6D82"
    }"#;

    let _user: User = serde_json::from_str(json_data).unwrap();
}

#[test]
fn test_deserialize_empty_string() {
    use serde::Deserialize;

    #[derive(Deserialize, Debug, PartialEq)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let json_data = r#"{}"#;

    let result: Result<User, serde_json::Error> = serde_json::from_str(json_data);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_not_a_json_object() {
    use serde::Deserialize;

    #[derive(Deserialize, Debug, PartialEq)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let json_data = r#""not an object""#;

    let result: Result<User, serde_json::Error> = serde_json::from_str(json_data);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_invalid_json() {
    use serde::Deserialize;

    #[derive(Deserialize, Debug, PartialEq)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let json_data = r#"
    {
        "fingerprint": "0xF9BA143B95FF6D82",
        "location": Menlo Park, CA
    }"#;

    let result: Result<User, serde_json::Error> = serde_json::from_str(json_data);
    assert!(result.is_err());
}

