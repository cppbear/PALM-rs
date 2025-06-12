// Answer 0

#[test]
fn test_deserialize_user() {
    use serde::Deserialize;
    use serde_json;

    #[derive(Deserialize, Debug)]
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
#[should_panic]
fn test_deserialize_user_missing_field() {
    use serde::Deserialize;
    use serde_json;

    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let json_data = r#"
        {
            "fingerprint": "0xF9BA143B95FF6D82"
        }"#; // Missing the "location" field

    let _: User = serde_json::from_str(json_data).unwrap();
}

#[test]
fn test_deserialize_user_invalid_json() {
    use serde::Deserialize;
    use serde_json;

    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let invalid_json_data = r#"
        {
            "fingerprint": "0xF9BA143B95FF6D82",
            "location": Menlo Park, CA
        }"#; // Invalid JSON format

    let result: Result<User, _> = serde_json::from_str(invalid_json_data);
    assert!(result.is_err());
}

