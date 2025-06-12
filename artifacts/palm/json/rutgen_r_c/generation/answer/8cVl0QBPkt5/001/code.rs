// Answer 0

#[test]
fn test_from_str_valid_user() {
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

    let user: User = from_str(json_data).unwrap();
    assert_eq!(user.fingerprint, "0xF9BA143B95FF6D82");
    assert_eq!(user.location, "Menlo Park, CA");
}

#[test]
#[should_panic]
fn test_from_str_invalid_json() {
    let json_data = r#"{"fingerprint": "0xF9BA143B95FF6D82", "location": "Menlo Park, CA""#; // Missing closing brace
    let _: Result<(), _> = from_str::<serde_json::Value>(json_data);
}

#[test]
#[should_panic]
fn test_from_str_missing_field() {
    // struct definition requires both fields
    #[derive(Deserialize, Debug, PartialEq)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let json_data = r#"
        {
            "fingerprint": "0xF9BA143B95FF6D82"
        }"#; // Missing location

    let _: User = from_str(json_data).unwrap(); // This should panic
}

#[test]
fn test_from_str_empty_input() {
    let json_data = r#""#; // Empty string
    let result: Result<serde_json::Value, _> = from_str(json_data);
    assert!(result.is_err());
}

#[test]
fn test_from_str_whitespace() {
    let json_data = r#"   "#; // Only whitespace
    let result: Result<serde_json::Value, _> = from_str(json_data);
    assert!(result.is_err());
}

#[test]
fn test_from_str_large_number() {
    let json_data = r#"
        {
            "fingerprint": "0xF9BA143B95FF6D82",
            "location": "Menlo Park, CA",
            "large_number": 1_000_000_000_000_000_000_000_000_000_000 // Too large for most types
        }"#;

    let result: Result<serde_json::Value, _> = from_str(json_data);
    assert!(result.is_err());
}

