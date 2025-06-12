// Answer 0

#[test]
fn test_eq_u64_with_value_as_number() {
    let number_value = Value::Number(Number::from(42));
    assert_eq!(eq_u64(&number_value, 42), true);
    assert_eq!(eq_u64(&number_value, 43), false);
}

#[test]
fn test_eq_u64_with_value_as_null() {
    let null_value = Value::Null;
    assert_eq!(eq_u64(&null_value, 0), false);
}

#[test]
fn test_eq_u64_with_value_as_bool() {
    let true_value = Value::Bool(true);
    let false_value = Value::Bool(false);
    assert_eq!(eq_u64(&true_value, 1), false);
    assert_eq!(eq_u64(&false_value, 0), false);
}

#[test]
fn test_eq_u64_with_value_as_string() {
    let string_value = Value::String(String::from("test"));
    assert_eq!(eq_u64(&string_value, 0), false);
}

#[test]
fn test_eq_u64_with_value_as_array() {
    let array_value = Value::Array(vec![Value::Number(Number::from(1)), Value::Number(Number::from(2))]);
    assert_eq!(eq_u64(&array_value, 0), false);
}

#[test]
fn test_eq_u64_with_value_as_object() {
    let object_value = Value::Object(Map::new());
    assert_eq!(eq_u64(&object_value, 0), false);
}

