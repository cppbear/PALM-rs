// Answer 0

#[test]
fn test_from_value_success() {
    use serde::Deserialize;
    use serde_json::{json, Value};

    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let j = json!({
        "fingerprint": "0xF9BA143B95FF6D82",
        "location": "Menlo Park, CA"
    });

    let result: User = from_value(j).unwrap();
    assert_eq!(result.fingerprint, "0xF9BA143B95FF6D82");
    assert_eq!(result.location, "Menlo Park, CA");
}

#[test]
#[should_panic]
fn test_from_value_missing_field() {
    use serde::Deserialize;
    use serde_json::{json, Value};

    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let j = json!({
        "fingerprint": "0xF9BA143B95FF6D82"
        // Missing location
    });

    let _: User = from_value(j).unwrap();
}

#[test]
#[should_panic]
fn test_from_value_wrong_type() {
    use serde::Deserialize;
    use serde_json::{json, Value};

    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let j = json!("Invalid JSON"); // This is a string, not a JSON map

    let _: User = from_value(j).unwrap();
}

#[test]
fn test_from_value_empty_json() {
    use serde::Deserialize;
    use serde_json::{json, Value};

    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let j = json!({}); // Empty JSON object

    let result: Result<User, _> = from_value(j);
    assert!(result.is_err()); // Expecting an error
}

#[test]
fn test_from_value_additional_fields() {
    use serde::Deserialize;
    use serde_json::{json, Value};

    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let j = json!({
        "fingerprint": "0xF9BA143B95FF6D82",
        "location": "Menlo Park, CA",
        "extra_field": "This should be ignored"
    });

    let result: User = from_value(j).unwrap();
    assert_eq!(result.fingerprint, "0xF9BA143B95FF6D82");
    assert_eq!(result.location, "Menlo Park, CA");
}

