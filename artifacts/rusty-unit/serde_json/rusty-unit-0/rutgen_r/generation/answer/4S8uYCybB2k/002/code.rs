// Answer 0

#[test]
fn test_as_u64_valid_integer() {
    use serde_json::{json, Value};
    
    let valid_integer = json!(64);
    assert_eq!(valid_integer.as_u64(), Some(64));

    let another_valid_integer = json!(0);
    assert_eq!(another_valid_integer.as_u64(), Some(0));
}

#[test]
fn test_as_u64_negative_integer() {
    use serde_json::{json, Value};
    
    let negative_integer = json!(-64);
    assert_eq!(negative_integer.as_u64(), None);
}

#[test]
fn test_as_u64_floating_point() {
    use serde_json::{json, Value};
    
    let floating_point = json!(256.0);
    assert_eq!(floating_point.as_u64(), None);

    let another_floating_point = json!(3.14);
    assert_eq!(another_floating_point.as_u64(), None);
}

#[test]
fn test_as_u64_non_number_types() {
    use serde_json::{json, Value};
    
    let string_value = json!("string");
    assert_eq!(string_value.as_u64(), None);

    let boolean_value = json!(true);
    assert_eq!(boolean_value.as_u64(), None);

    let null_value = json!(null);
    assert_eq!(null_value.as_u64(), None);

    let array_value = json!([1, 2, 3]);
    assert_eq!(array_value.as_u64(), None);

    let object_value = json!({"key": "value"});
    assert_eq!(object_value.as_u64(), None);
}

