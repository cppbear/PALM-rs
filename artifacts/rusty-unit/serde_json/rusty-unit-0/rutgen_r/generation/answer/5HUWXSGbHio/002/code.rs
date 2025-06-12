// Answer 0

#[test]
fn test_fmt_array() {
    use serde_json::Value;
    use std::fmt;

    struct TestStruct(Value);

    impl fmt::Display for TestStruct {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match *self.0 {
                Value::Null => formatter.write_str("null"),
                Value::Bool(_) => formatter.write_str("boolean"),
                Value::Number(_) => formatter.write_str("number"),
                Value::String(_) => formatter.write_str("string"),
                Value::Array(_) => formatter.write_str("array"),
                Value::Object(_) => formatter.write_str("object"),
            }
        }
    }

    let value_array = Value::Array(vec![Value::String("test".to_string()), Value::Number(42.into())]);
    let test_struct = TestStruct(value_array);
    
    let mut output = String::new();
    test_struct.fmt(&mut fmt::Formatter::new(&mut output)).unwrap();
    
    assert_eq!(output, "array");
}

#[test]
fn test_fmt_null() {
    use serde_json::Value;
    use std::fmt;

    struct TestStruct(Value);

    impl fmt::Display for TestStruct {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match *self.0 {
                Value::Null => formatter.write_str("null"),
                Value::Bool(_) => formatter.write_str("boolean"),
                Value::Number(_) => formatter.write_str("number"),
                Value::String(_) => formatter.write_str("string"),
                Value::Array(_) => formatter.write_str("array"),
                Value::Object(_) => formatter.write_str("object"),
            }
        }
    }

    let value_null = Value::Null;
    let test_struct = TestStruct(value_null);
    
    let mut output = String::new();
    test_struct.fmt(&mut fmt::Formatter::new(&mut output)).unwrap();
    
    assert_eq!(output, "null");
} 

#[test]
fn test_fmt_bool() {
    use serde_json::Value;
    use std::fmt;

    struct TestStruct(Value);

    impl fmt::Display for TestStruct {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match *self.0 {
                Value::Null => formatter.write_str("null"),
                Value::Bool(_) => formatter.write_str("boolean"),
                Value::Number(_) => formatter.write_str("number"),
                Value::String(_) => formatter.write_str("string"),
                Value::Array(_) => formatter.write_str("array"),
                Value::Object(_) => formatter.write_str("object"),
            }
        }
    }

    let value_bool = Value::Bool(true);
    let test_struct = TestStruct(value_bool);
    
    let mut output = String::new();
    test_struct.fmt(&mut fmt::Formatter::new(&mut output)).unwrap();
    
    assert_eq!(output, "boolean");
}

#[test]
fn test_fmt_number() {
    use serde_json::Value;
    use std::fmt;

    struct TestStruct(Value);

    impl fmt::Display for TestStruct {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match *self.0 {
                Value::Null => formatter.write_str("null"),
                Value::Bool(_) => formatter.write_str("boolean"),
                Value::Number(_) => formatter.write_str("number"),
                Value::String(_) => formatter.write_str("string"),
                Value::Array(_) => formatter.write_str("array"),
                Value::Object(_) => formatter.write_str("object"),
            }
        }
    }

    let value_number = Value::Number(10.into());
    let test_struct = TestStruct(value_number);
    
    let mut output = String::new();
    test_struct.fmt(&mut fmt::Formatter::new(&mut output)).unwrap();
    
    assert_eq!(output, "number");
}

#[test]
fn test_fmt_string() {
    use serde_json::Value;
    use std::fmt;

    struct TestStruct(Value);

    impl fmt::Display for TestStruct {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match *self.0 {
                Value::Null => formatter.write_str("null"),
                Value::Bool(_) => formatter.write_str("boolean"),
                Value::Number(_) => formatter.write_str("number"),
                Value::String(_) => formatter.write_str("string"),
                Value::Array(_) => formatter.write_str("array"),
                Value::Object(_) => formatter.write_str("object"),
            }
        }
    }

    let value_string = Value::String("Hello".to_string());
    let test_struct = TestStruct(value_string);
    
    let mut output = String::new();
    test_struct.fmt(&mut fmt::Formatter::new(&mut output)).unwrap();
    
    assert_eq!(output, "string");
}

#[test]
fn test_fmt_object() {
    use serde_json::Value;
    use std::fmt;

    struct TestStruct(Value);

    impl fmt::Display for TestStruct {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match *self.0 {
                Value::Null => formatter.write_str("null"),
                Value::Bool(_) => formatter.write_str("boolean"),
                Value::Number(_) => formatter.write_str("number"),
                Value::String(_) => formatter.write_str("string"),
                Value::Array(_) => formatter.write_str("array"),
                Value::Object(_) => formatter.write_str("object"),
            }
        }
    }

    let value_object = Value::Object(serde_json::map::Map::new());
    let test_struct = TestStruct(value_object);
    
    let mut output = String::new();
    test_struct.fmt(&mut fmt::Formatter::new(&mut output)).unwrap();
    
    assert_eq!(output, "object");
}

