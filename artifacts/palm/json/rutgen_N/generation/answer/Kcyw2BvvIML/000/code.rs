// Answer 0

#[test]
fn test_to_value_for_struct() {
    use serde::Serialize;
    use serde_json::json;
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

    let v = serde_json::to_value(u).unwrap();
    assert_eq!(v, expected);
}

#[test]
#[should_panic]
fn test_to_value_for_non_serializable_keys() {
    use std::collections::BTreeMap;

    let mut map = BTreeMap::new();
    map.insert(vec![32, 64], "x86");

    let _ = serde_json::to_value(map).unwrap();
}

