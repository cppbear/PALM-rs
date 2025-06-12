// Answer 0

#[test]
fn test_serialize_newtype_struct_with_string() {
    struct TestValue(String);
    
    impl Serialize for TestValue {
        fn serialize<S>(&self, serializer: S) -> Result<Value>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(&self.0)
        }
    }
    
    let serializer = Serializer;
    let value = TestValue(String::from("test string"));
    let result = serializer.serialize_newtype_struct("Test", &value).unwrap();
    assert_eq!(result, Value::String(String::from("test string")));
}

#[test]
fn test_serialize_newtype_struct_with_bool() {
    struct TestBool(bool);
    
    impl Serialize for TestBool {
        fn serialize<S>(&self, serializer: S) -> Result<Value>
        where
            S: serde::Serializer,
        {
            serializer.serialize_bool(self.0)
        }
    }
    
    let serializer = Serializer;
    let value = TestBool(true);
    let result = serializer.serialize_newtype_struct("TestBool", &value).unwrap();
    assert_eq!(result, Value::Bool(true));
}

#[test]
fn test_serialize_newtype_struct_with_number() {
    struct TestNumber(i32);
    
    impl Serialize for TestNumber {
        fn serialize<S>(&self, serializer: S) -> Result<Value>
        where
            S: serde::Serializer,
        {
            serializer.serialize_i32(self.0)
        }
    }
    
    let serializer = Serializer;
    let value = TestNumber(42);
    let result = serializer.serialize_newtype_struct("TestNumber", &value).unwrap();
    assert_eq!(result, Value::Number(42.into()));
}

#[test]
fn test_serialize_newtype_struct_with_none_value() {
    struct TestNone;
    
    impl Serialize for TestNone {
        fn serialize<S>(&self, serializer: S) -> Result<Value>
        where
            S: serde::Serializer,
        {
            serializer.serialize_unit()
        }
    }
    
    let serializer = Serializer;
    let value = TestNone;
    let result = serializer.serialize_newtype_struct("TestNone", &value).unwrap();
    assert_eq!(result, Value::Null);
}

