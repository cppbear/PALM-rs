// Answer 0

#[derive(Debug)]
enum Value {
    Null,
    Bool(bool),
    Number(i32),
    String(String),
    Array(Vec<Value>),
    Object(std::collections::HashMap<String, Value>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Null => formatter.write_str("Null"),
            Value::Bool(boolean) => write!(formatter, "Bool({})", boolean),
            Value::Number(number) => write!(formatter, "Number({})", number),
            Value::String(string) => write!(formatter, "String({:?})", string),
            Value::Array(vec) => {
                formatter.write_str("Array ")?;
                for (i, v) in vec.iter().enumerate() {
                    if i > 0 {
                        formatter.write_str(", ")?;
                    }
                    v.fmt(formatter)?;
                }
                Ok(())
            }
            Value::Object(map) => {
                formatter.write_str("Object ")?;
                for (key, value) in map {
                    write!(formatter, "{}: {:?}, ", key, value)?;
                }
                Ok(())
            }
        }
    }
}

#[test]
fn test_value_object() {
    let mut map = std::collections::HashMap::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.insert("key2".to_string(), Value::Number(42));
    
    let value = Value::Object(map);
    let mut formatter = std::fmt::Formatter::new();
    
    assert_eq!(value.fmt(&mut formatter).is_ok(), true);
}

#[test]
fn test_value_array() {
    let value = Value::Array(vec![
        Value::Bool(true),
        Value::Number(5),
        Value::String("test".to_string()),
    ]);
    
    let mut formatter = std::fmt::Formatter::new();
    
    assert_eq!(value.fmt(&mut formatter).is_ok(), true);
} 

#[test]
fn test_value_object_empty() {
    let map: std::collections::HashMap<String, Value> = std::collections::HashMap::new();
    let value = Value::Object(map);
    let mut formatter = std::fmt::Formatter::new();
    
    assert_eq!(value.fmt(&mut formatter).is_ok(), true);
}

