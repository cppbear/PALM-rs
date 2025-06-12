// Answer 0

#[test]
fn test_empty_json() {
    from_str::<serde_json::Value>("{}");
}

#[test]
fn test_empty_user_json() {
    from_str::<User>("{\"fingerprint\": \"\", \"location\": \"\" }");
}

#[test]
fn test_valid_user_json() {
    from_str::<User>(r#"{"fingerprint": "0xF9BA143B95FF6D82", "location": "Menlo Park, CA"}"#);
}

#[should_panic]
fn test_invalid_fingerprint_user_json() {
    from_str::<User>(r#"{"fingerprint": "0xGHIJKL", "location": "Menlo Park, CA"}"#);
}

#[should_panic]
fn test_missing_location_user_json() {
    from_str::<User>(r#"{"fingerprint": "0xF9BA143B95FF6D82", "location": ""}"#);
}

#[should_panic]
fn test_large_fingerprint_user_json() {
    from_str::<User>(r#"{"fingerprint": "1234567890123456789012345678901234567890", "location": "Valid Location"}"#);
}

#[should_panic]
fn test_non_string_location_user_json() {
    from_str::<User>(r#"{"fingerprint": "0xF9BA143B95FF6D82", "location": "Menlo Park, 123.0"}"#);
}

#[should_panic]
fn test_array_instead_of_object() {
    from_str::<User>(" [1, 2, 3]");
}

#[should_panic]
fn test_empty_string() {
    from_str::<User>("\"\"");
}

