// Answer 0

#[test]
fn test_display_type_string() {
    use super::Value;
    use alloc::string::String;
    
    let value = Value::String(String::from("a test string"));
    let type_instance = Type(&value);
    
    let mut output = String::new();
    let result = type_instance.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "string");
}

#[test]
fn test_display_type_null() {
    use super::Value;
    
    let value = Value::Null;
    let type_instance = Type(&value);
    
    let mut output = String::new();
    let result = type_instance.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "null");
}

#[test]
fn test_display_type_bool() {
    use super::Value;

    let value = Value::Bool(true);
    let type_instance = Type(&value);
    
    let mut output = String::new();
    let result = type_instance.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "boolean");
}

#[test]
fn test_display_type_number() {
    use super::Value;

    let value = Value::Number(Number::from(42));
    let type_instance = Type(&value);
    
    let mut output = String::new();
    let result = type_instance.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "number");
}

#[test]
fn test_display_type_array() {
    use super::Value;

    let value = Value::Array(vec![Value::String(String::from("test"))]);
    let type_instance = Type(&value);
    
    let mut output = String::new();
    let result = type_instance.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "array");
}

#[test]
fn test_display_type_object() {
    use super::Value;
    use crate::map::Map;

    let obj = Map::new();
    let value = Value::Object(obj);
    let type_instance = Type(&value);
    
    let mut output = String::new();
    let result = type_instance.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "object");
}

