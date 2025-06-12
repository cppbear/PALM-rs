// Answer 0

#[test]
fn test_serialize_u32() {
    struct Serializer {
        value: Option<u64>,
    }

    impl Serializer {
        fn serialize_u64(&mut self, value: u64) -> Result<Value, &'static str> {
            self.value = Some(value);
            Ok(Value::U64(value))
        }

        fn serialize_u32(self, value: u32) -> Result<Value, &'static str> {
            self.serialize_u64(value as u64)
        }
    }

    #[derive(Debug)]
    enum Value {
        U64(u64),
    }

    let mut serializer = Serializer { value: None };
    let result = serializer.serialize_u32(42);
    
    match result {
        Ok(Value::U64(val)) => assert_eq!(val, 42 as u64),
        _ => panic!("Expected Ok(Value::U64), but got {:?}", result),
    }
}

#[test]
fn test_serialize_u32_boundary() {
    struct Serializer {
        value: Option<u64>,
    }

    impl Serializer {
        fn serialize_u64(&mut self, value: u64) -> Result<Value, &'static str> {
            self.value = Some(value);
            Ok(Value::U64(value))
        }

        fn serialize_u32(self, value: u32) -> Result<Value, &'static str> {
            self.serialize_u64(value as u64)
        }
    }

    #[derive(Debug)]
    enum Value {
        U64(u64),
    }

    let mut serializer = Serializer { value: None };
    let result_min = serializer.serialize_u32(0);
    let result_max = serializer.serialize_u32(u32::MAX);

    match result_min {
        Ok(Value::U64(val)) => assert_eq!(val, 0),
        _ => panic!("Expected Ok(Value::U64) for min, but got {:?}", result_min),
    }

    match result_max {
        Ok(Value::U64(val)) => assert_eq!(val, u32::MAX as u64),
        _ => panic!("Expected Ok(Value::U64) for max, but got {:?}", result_max),
    }
}

