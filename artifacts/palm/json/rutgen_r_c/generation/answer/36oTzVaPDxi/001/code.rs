// Answer 0

#[test]
fn test_from_value_with_valid_user() {
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
fn test_from_value_with_missing_field() {
    use serde::Deserialize;
    use serde_json::json;

    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let j = json!({
        "fingerprint": "0xF9BA143B95FF6D82",
    });

    let _: User = from_value(j).unwrap();
}

#[test]
#[should_panic]
fn test_from_value_with_incorrect_type() {
    use serde::Deserialize;
    use serde_json::json;

    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let j = json!(42); // Incorrect type, should panic

    let _: User = from_value(j).unwrap();
}

#[test]
fn test_from_value_with_empty_object() {
    use serde::Deserialize;
    use serde_json::json;

    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let j = json!({}); // Should not panic but return an error

    let result: Result<User, Error> = from_value(j);
    assert!(result.is_err());
}

#[test]
fn test_from_value_with_valid_array() {
    use serde::Deserialize;
    use serde_json::json;

    #[derive(Deserialize, Debug, PartialEq)]
    struct UserArray {
        users: Vec<User>,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let j = json!({
        "users": [
            {
                "fingerprint": "0xF9BA143B95FF6D82",
                "location": "Menlo Park, CA"
            },
            {
                "fingerprint": "0x1234567890ABCDEF",
                "location": "Mountain View, CA"
            }
        ]
    });

    let user_array: UserArray = from_value(j).unwrap();
    assert_eq!(user_array.users.len(), 2);
    assert_eq!(user_array.users[0].fingerprint, "0xF9BA143B95FF6D82");
    assert_eq!(user_array.users[1].location, "Mountain View, CA");
}

