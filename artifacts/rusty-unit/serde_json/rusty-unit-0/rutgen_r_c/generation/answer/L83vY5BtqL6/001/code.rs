// Answer 0

#[test]
fn test_as_object_mut_with_null() {
    let mut value = Value::Null;
    assert_eq!(value.as_object_mut(), None);
}

#[test]
fn test_as_object_mut_with_bool() {
    let mut value = Value::Bool(true);
    assert_eq!(value.as_object_mut(), None);
}

#[test]
fn test_as_object_mut_with_number() {
    let mut value = Value::Number(Number { n: 42 });
    assert_eq!(value.as_object_mut(), None);
}

#[test]
fn test_as_object_mut_with_string() {
    let mut value = Value::String(String::from("test"));
    assert_eq!(value.as_object_mut(), None);
}

#[test]
fn test_as_object_mut_with_array() {
    let mut value = Value::Array(vec![Value::String(String::from("item1")), Value::String(String::from("item2"))]);
    assert_eq!(value.as_object_mut(), None);
}

