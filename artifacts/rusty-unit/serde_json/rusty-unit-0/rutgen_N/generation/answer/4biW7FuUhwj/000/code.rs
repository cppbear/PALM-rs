// Answer 0

#[test]
fn test_is_number_true() {
    use serde_json::{json, Value};

    let v = json!({ "a": 1 });
    
    assert!(v["a"].is_number());
}

#[test]
fn test_is_number_false() {
    use serde_json::{json, Value};

    let v = json!({ "b": "2" });

    assert!(!v["b"].is_number());
}

#[test]
fn test_is_number_with_floating_point() {
    use serde_json::{json, Value};

    let v = json!({ "c": 1.5 });

    assert!(v["c"].is_number());
}

#[test]
fn test_is_number_with_array() {
    use serde_json::{json, Value};

    let v = json!([1, 2, "3"]);

    assert!(v[0].is_number());
    assert!(!v[2].is_number());
}

#[test]
fn test_is_number_with_boolean() {
    use serde_json::{json, Value};

    let v = json!({ "d": true });

    assert!(!v["d"].is_number());
}

#[test]
fn test_is_number_with_null() {
    use serde_json::{json, Value};

    let v = json!({ "e": null });

    assert!(!v["e"].is_number());
}

