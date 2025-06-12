// Answer 0

#[test]
fn test_to_value_success() {
    use serde::Serialize;
    use serde_json::json;
    use serde_json::Value;
    use std::error::Error;

    #[derive(Serialize)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let u = User {
        fingerprint: "0xF9BA143B95FF6D82".to_owned(),
        location: "Menlo Park, CA".to_owned(),
    };

    let expected = json!({
        "fingerprint": "0xF9BA143B95FF6D82",
        "location": "Menlo Park, CA",
    });

    let v: Value = serde_json::to_value(u).unwrap();
    assert_eq!(v, expected);
}

#[test]
#[should_panic]
fn test_to_value_non_string_map_key() {
    use std::collections::BTreeMap;

    let mut map = BTreeMap::new();
    map.insert(vec![32, 64], "x86"); // Non-string key

    let _v = serde_json::to_value(map).unwrap(); // This should panic
}

#[test]
fn test_to_value_empty_struct() {
    use serde::Serialize;
    use serde_json::json;
    use serde_json::Value;

    #[derive(Serialize)]
    struct Empty;

    let e = Empty;

    let expected = json!({}); // Expecting an empty JSON object

    let v: Value = serde_json::to_value(e).unwrap();
    assert_eq!(v, expected);
}

#[test]
fn test_to_value_nested_structs() {
    use serde::Serialize;
    use serde_json::json;
    use serde_json::Value;

    #[derive(Serialize)]
    struct Address {
        city: String,
        state: String,
    }

    #[derive(Serialize)]
    struct User {
        fingerprint: String,
        location: Address,
    }

    let user = User {
        fingerprint: "0xF9BA143B95FF6D82".to_owned(),
        location: Address {
            city: "Menlo Park".to_owned(),
            state: "CA".to_owned(),
        },
    };

    let expected = json!({
        "fingerprint": "0xF9BA143B95FF6D82",
        "location": {
            "city": "Menlo Park",
            "state": "CA",
        },
    });

    let v: Value = serde_json::to_value(user).unwrap();
    assert_eq!(v, expected);
}

