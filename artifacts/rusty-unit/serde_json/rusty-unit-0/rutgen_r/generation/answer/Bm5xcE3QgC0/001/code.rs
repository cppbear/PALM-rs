// Answer 0

#[test]
fn test_eq_bool_true() {
    let value = serde_json::json!(true);
    let other = true;
    assert_eq!(eq_bool(&value, other), true);
}

#[test]
fn test_eq_bool_false() {
    let value = serde_json::json!(false);
    let other = false;
    assert_eq!(eq_bool(&value, other), true);
}

#[test]
fn test_eq_bool_none() {
    let value = serde_json::json!(null);
    let other = true;
    assert_eq!(eq_bool(&value, other), false);
}

#[test]
fn test_eq_bool_number() {
    let value = serde_json::json!(1);
    let other = true;
    assert_eq!(eq_bool(&value, other), false);
}

#[test]
fn test_eq_bool_string() {
    let value = serde_json::json!("true");
    let other = true;
    assert_eq!(eq_bool(&value, other), false);
}

#[test]
fn test_eq_bool_empty_array() {
    let value = serde_json::json!([]);
    let other = false;
    assert_eq!(eq_bool(&value, other), false);
}

#[test]
fn test_eq_bool_empty_object() {
    let value = serde_json::json!({});
    let other = true;
    assert_eq!(eq_bool(&value, other), false);
}

