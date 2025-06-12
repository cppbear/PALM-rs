// Answer 0

#[test]
fn test_as_number_with_integer() {
    use serde_json::{json, Number, Value};

    let v = json!({ "a": 1 });
    assert_eq!(v["a"].as_number(), Some(&Number::from(1u64)));
}

#[test]
fn test_as_number_with_float() {
    use serde_json::{json, Number, Value};

    let v = json!({ "b": 2.2 });
    assert_eq!(v["b"].as_number(), Some(&Number::from_f64(2.2).unwrap()));
}

#[test]
fn test_as_number_with_negative_integer() {
    use serde_json::{json, Number, Value};

    let v = json!({ "c": -3 });
    assert_eq!(v["c"].as_number(), Some(&Number::from(-3i64)));
}

#[test]
fn test_as_number_with_string() {
    use serde_json::{json, Value};

    let v = json!({ "d": "4" });
    assert_eq!(v["d"].as_number(), None);
}

