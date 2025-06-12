// Answer 0

#[test]
fn test_as_array_mut_not_array() {
    let mut value_null = Value::Null;
    assert_eq!(value_null.as_array_mut(), None);
    
    let mut value_bool = Value::Bool(true);
    assert_eq!(value_bool.as_array_mut(), None);
    
    let mut value_number = Value::Number(Number { n: 0 });
    assert_eq!(value_number.as_array_mut(), None);
    
    let mut value_string = Value::String(String::from("a string"));
    assert_eq!(value_string.as_array_mut(), None);
    
    let mut value_object = Value::Object(Map { map: MapImpl::new() });
    assert_eq!(value_object.as_array_mut(), None);
}

#[test]
fn test_as_array_mut_with_array() {
    let mut value_array = Value::Array(vec![Value::String(String::from("first")), Value::String(String::from("second"))]);
    let array_mut = value_array.as_array_mut().unwrap();
    array_mut.push(Value::String(String::from("third")));
    assert_eq!(array_mut.len(), 3);
}

#[test]
fn test_as_array_mut_on_empty_array() {
    let mut empty_array = Value::Array(vec![]);
    let array_mut = empty_array.as_array_mut().unwrap();
    array_mut.push(Value::String(String::from("new element")));
    assert_eq!(array_mut.len(), 1);
}

