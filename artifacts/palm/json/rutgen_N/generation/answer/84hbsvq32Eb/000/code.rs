// Answer 0

#[test]
fn test_deserialize_user() {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let j = b"
        {
            \"fingerprint\": \"0xF9BA143B95FF6D82\",
            \"location\": \"Menlo Park, CA\"
        }";

    let u: User = serde_json::from_slice(j).unwrap();
    assert_eq!(u.fingerprint, "0xF9BA143B95FF6D82");
    assert_eq!(u.location, "Menlo Park, CA");
}

#[test]
#[should_panic]
fn test_deserialize_user_missing_field() {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let j = b"
        {
            \"fingerprint\": \"0xF9BA143B95FF6D82\"
        }"; // Missing location

    let _: User = serde_json::from_slice(j).unwrap(); // Should panic
}

#[test]
fn test_deserialize_user_empty_json() {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let j = b"{}"; // Empty JSON

    let result: Result<User, _> = serde_json::from_slice(j);
    assert!(result.is_err());
}

