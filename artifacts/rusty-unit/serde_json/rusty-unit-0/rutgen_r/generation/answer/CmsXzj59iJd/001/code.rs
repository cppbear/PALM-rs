// Answer 0

#[test]
fn test_serialize_i32_with_positive_value() {
    struct TestSerializer;
    
    impl TestSerializer {
        fn serialize_i64(&self, value: i64) -> Result<Value> {
            Ok(Value::Number(serde_json::Number::from(value)))
        }
        
        fn serialize_i32(self, value: i32) -> Result<Value> {
            self.serialize_i64(value as i64)
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_i32(42);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Number(serde_json::Number::from(42)));
}

#[test]
fn test_serialize_i32_with_negative_value() {
    struct TestSerializer;
    
    impl TestSerializer {
        fn serialize_i64(&self, value: i64) -> Result<Value> {
            Ok(Value::Number(serde_json::Number::from(value)))
        }
        
        fn serialize_i32(self, value: i32) -> Result<Value> {
            self.serialize_i64(value as i64)
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_i32(-42);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Number(serde_json::Number::from(-42)));
}

#[test]
fn test_serialize_i32_with_zero() {
    struct TestSerializer;
    
    impl TestSerializer {
        fn serialize_i64(&self, value: i64) -> Result<Value> {
            Ok(Value::Number(serde_json::Number::from(value)))
        }
        
        fn serialize_i32(self, value: i32) -> Result<Value> {
            self.serialize_i64(value as i64)
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_i32(0);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Number(serde_json::Number::from(0)));
}

#[test]
fn test_serialize_i32_with_minimum_i32() {
    struct TestSerializer;
    
    impl TestSerializer {
        fn serialize_i64(&self, value: i64) -> Result<Value> {
            Ok(Value::Number(serde_json::Number::from(value)))
        }
        
        fn serialize_i32(self, value: i32) -> Result<Value> {
            self.serialize_i64(value as i64)
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_i32(i32::MIN);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Number(serde_json::Number::from(i32::MIN as i64)));
}

#[test]
fn test_serialize_i32_with_maximum_i32() {
    struct TestSerializer;
    
    impl TestSerializer {
        fn serialize_i64(&self, value: i64) -> Result<Value> {
            Ok(Value::Number(serde_json::Number::from(value)))
        }

        fn serialize_i32(self, value: i32) -> Result<Value> {
            self.serialize_i64(value as i64)
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_i32(i32::MAX);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Number(serde_json::Number::from(i32::MAX as i64)));
}

