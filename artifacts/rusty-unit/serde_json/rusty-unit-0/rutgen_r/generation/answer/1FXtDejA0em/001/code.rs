// Answer 0

#[test]
fn test_serialize_i8_valid_value() {
    struct Serializer;

    impl Serializer {
        fn serialize_i64(&self, value: i64) -> Result<Value> {
            Ok(Value::Number(serde_json::Number::from(value)))
        }
        
        fn serialize_i8(self, value: i8) -> Result<Value> {
            self.serialize_i64(value as i64)
        }
    }

    let serializer = Serializer;
    let result = serializer.serialize_i8(10);
    assert_eq!(result, Ok(Value::Number(serde_json::Number::from(10))));
}

#[test]
fn test_serialize_i8_zero() {
    struct Serializer;

    impl Serializer {
        fn serialize_i64(&self, value: i64) -> Result<Value> {
            Ok(Value::Number(serde_json::Number::from(value)))
        }
        
        fn serialize_i8(self, value: i8) -> Result<Value> {
            self.serialize_i64(value as i64)
        }
    }

    let serializer = Serializer;
    let result = serializer.serialize_i8(0);
    assert_eq!(result, Ok(Value::Number(serde_json::Number::from(0))));
}

#[test]
fn test_serialize_i8_negative_value() {
    struct Serializer;

    impl Serializer {
        fn serialize_i64(&self, value: i64) -> Result<Value> {
            Ok(Value::Number(serde_json::Number::from(value)))
        }
        
        fn serialize_i8(self, value: i8) -> Result<Value> {
            self.serialize_i64(value as i64)
        }
    }

    let serializer = Serializer;
    let result = serializer.serialize_i8(-10);
    assert_eq!(result, Ok(Value::Number(serde_json::Number::from(-10))));
}

#[test]
fn test_serialize_i8_max_value() {
    struct Serializer;

    impl Serializer {
        fn serialize_i64(&self, value: i64) -> Result<Value> {
            Ok(Value::Number(serde_json::Number::from(value)))
        }
        
        fn serialize_i8(self, value: i8) -> Result<Value> {
            self.serialize_i64(value as i64)
        }
    }

    let serializer = Serializer;
    let result = serializer.serialize_i8(i8::MAX);
    assert_eq!(result, Ok(Value::Number(serde_json::Number::from(i8::MAX as i64))));
}

#[test]
fn test_serialize_i8_min_value() {
    struct Serializer;

    impl Serializer {
        fn serialize_i64(&self, value: i64) -> Result<Value> {
            Ok(Value::Number(serde_json::Number::from(value)))
        }
        
        fn serialize_i8(self, value: i8) -> Result<Value> {
            self.serialize_i64(value as i64)
        }
    }

    let serializer = Serializer;
    let result = serializer.serialize_i8(i8::MIN);
    assert_eq!(result, Ok(Value::Number(serde_json::Number::from(i8::MIN as i64))));
}

