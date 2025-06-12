// Answer 0

#[test]
fn test_from_value_valid_user() {
    use serde::Deserialize;
    use serde_json::json;

    #[derive(Deserialize, Debug, PartialEq)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let j = json!({
        "fingerprint": "0xF9BA143B95FF6D82",
        "location": "Menlo Park, CA"
    });

    let user: User = from_value(j).unwrap();
    assert_eq!(user, User {
        fingerprint: "0xF9BA143B95FF6D82".to_string(),
        location: "Menlo Park, CA".to_string(),
    });
}

#[test]
#[should_panic]
fn test_from_value_missing_field() {
    use serde::Deserialize;
    use serde_json::json;

    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let j = json!({
        "fingerprint": "0xF9BA143B95FF6D82"
        // Missing "location"
    });

    let _: User = from_value(j).unwrap();
}

#[test]
#[should_panic]
fn test_from_value_incorrect_type() {
    use serde::Deserialize;
    use serde_json::json;

    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let j = json!({
        "fingerprint": 12345, // Incorrect type for "fingerprint"
        "location": "Menlo Park, CA"
    });

    let _: User = from_value(j).unwrap();
}

#[test]
fn test_from_value_empty_object() {
    use serde::Deserialize;
    use serde_json::json;

    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let j = json!({});

    let result: Result<User, _> = from_value(j);
    assert!(result.is_err());
}

#[test]
fn test_from_value_additional_fields() {
    use serde::Deserialize;
    use serde_json::json;

    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let j = json!({
        "fingerprint": "0xF9BA143B95FF6D82",
        "location": "Menlo Park, CA",
        "extra_field": "should be ignored"
    });

    let user: User = from_value(j).unwrap();
    assert_eq!(user, User {
        fingerprint: "0xF9BA143B95FF6D82".to_string(),
        location: "Menlo Park, CA".to_string(),
    });
}

