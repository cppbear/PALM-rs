// Answer 0

#[test]
fn test_fmt_null_value() {
    struct ValueWrapper(Value);
    
    enum Value {
        Null,
        Bool(bool),
        Number(i32),
        String(String),
        Array(Vec<Value>),
        Object(std::collections::HashMap<String, Value>),
    }
    
    let wrapper = ValueWrapper(Value::Null);
    let mut buffer = std::fmt::Formatter::new();
    
    assert_eq!(wrapper.fmt(&mut buffer).unwrap(), "null");
}

#[test]
fn test_fmt_boolean_value() {
    struct ValueWrapper(Value);
    
    enum Value {
        Null,
        Bool(bool),
        Number(i32),
        String(String),
        Array(Vec<Value>),
        Object(std::collections::HashMap<String, Value>),
    }
    
    let wrapper = ValueWrapper(Value::Bool(true));
    let mut buffer = std::fmt::Formatter::new();
    
    assert_eq!(wrapper.fmt(&mut buffer).unwrap(), "boolean");
}

#[test]
fn test_fmt_number_value() {
    struct ValueWrapper(Value);
    
    enum Value {
        Null,
        Bool(bool),
        Number(i32),
        String(String),
        Array(Vec<Value>),
        Object(std::collections::HashMap<String, Value>),
    }
    
    let wrapper = ValueWrapper(Value::Number(42));
    let mut buffer = std::fmt::Formatter::new();
    
    assert_eq!(wrapper.fmt(&mut buffer).unwrap(), "number");
}

#[test]
fn test_fmt_string_value() {
    struct ValueWrapper(Value);
    
    enum Value {
        Null,
        Bool(bool),
        Number(i32),
        String(String),
        Array(Vec<Value>),
        Object(std::collections::HashMap<String, Value>),
    }
    
    let wrapper = ValueWrapper(Value::String("test".to_string()));
    let mut buffer = std::fmt::Formatter::new();
    
    assert_eq!(wrapper.fmt(&mut buffer).unwrap(), "string");
}

#[test]
fn test_fmt_array_value() {
    struct ValueWrapper(Value);
    
    enum Value {
        Null,
        Bool(bool),
        Number(i32),
        String(String),
        Array(Vec<Value>),
        Object(std::collections::HashMap<String, Value>),
    }
    
    let wrapper = ValueWrapper(Value::Array(vec![]));
    let mut buffer = std::fmt::Formatter::new();
    
    assert_eq!(wrapper.fmt(&mut buffer).unwrap(), "array");
}

#[test]
fn test_fmt_object_value() {
    struct ValueWrapper(Value);
    
    enum Value {
        Null,
        Bool(bool),
        Number(i32),
        String(String),
        Array(Vec<Value>),
        Object(std::collections::HashMap<String, Value>),
    }

    let wrapper = ValueWrapper(Value::Object(std::collections::HashMap::new()));
    let mut buffer = std::fmt::Formatter::new();

    assert_eq!(wrapper.fmt(&mut buffer).unwrap(), "object");
}

