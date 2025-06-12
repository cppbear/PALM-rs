// Answer 0

#[test]
fn test_is_number_with_number_value() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!(42);
    assert!(v.is_number());

    let v = json!(3.14);
    assert!(v.is_number());

    let v = json!(0);
    assert!(v.is_number());

    let v = json!(-1);
    assert!(v.is_number());

    let v = json!(1.0);
    assert!(v.is_number());
}

#[test]
fn test_is_number_with_non_number_values() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!("string");
    assert!(!v.is_number());

    let v = json!(true);
    assert!(!v.is_number());

    let v = json!(null);
    assert!(!v.is_number());

    let v = json!({});
    assert!(!v.is_number());

    let v = json!([]);
    assert!(!v.is_number());
}

