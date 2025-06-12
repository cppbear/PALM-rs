// Answer 0

#[test]
fn test_from_value_success() {
    use serde::Deserialize;
    use serde_json::{json, Value, Error};

    #[derive(Deserialize, Debug, PartialEq)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let j: Value = json!({
        "fingerprint": "0xF9BA143B95FF6D82",
        "location": "Menlo Park, CA"
    });

    let result: User = from_value(j).unwrap();
    assert_eq!(
        result,
        User {
            fingerprint: "0xF9BA143B95FF6D82".to_string(),
            location: "Menlo Park, CA".to_string(),
        }
    );
}

#[test]
fn test_from_value_missing_field() {
    use serde::Deserialize;
    use serde_json::{json, Value, Error};

    #[derive(Deserialize, Debug, PartialEq)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let j: Value = json!({
        "fingerprint": "0xF9BA143B95FF6D82",
        // "location" is missing
    });

    let result: Result<User, Error> = from_value(j);
    assert!(result.is_err());
}

#[test]
fn test_from_value_incorrect_type() {
    use serde::Deserialize;
    use serde_json::{json, Value, Error};

    #[derive(Deserialize, Debug, PartialEq)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let j: Value = json!(42); // Not a JSON object

    let result: Result<User, Error> = from_value(j);
    assert!(result.is_err());
}

