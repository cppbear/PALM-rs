// Answer 0

#[test]
fn test_serialize_i8_positive_value() {
    struct Serializer;

    impl Serializer {
        fn serialize_i64(&self, value: i64) -> Result<Value> {
            Ok(Value::from(value))
        }

        fn serialize_i8(self, value: i8) -> Result<Value> {
            self.serialize_i64(value as i64)
        }
    }

    let serializer = Serializer;
    let result = serializer.serialize_i8(127);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::from(127));
}

#[test]
fn test_serialize_i8_negative_value() {
    struct Serializer;

    impl Serializer {
        fn serialize_i64(&self, value: i64) -> Result<Value> {
            Ok(Value::from(value))
        }

        fn serialize_i8(self, value: i8) -> Result<Value> {
            self.serialize_i64(value as i64)
        }
    }

    let serializer = Serializer;
    let result = serializer.serialize_i8(-128);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::from(-128));
}

#[test]
fn test_serialize_i8_zero() {
    struct Serializer;

    impl Serializer {
        fn serialize_i64(&self, value: i64) -> Result<Value> {
            Ok(Value::from(value))
        }

        fn serialize_i8(self, value: i8) -> Result<Value> {
            self.serialize_i64(value as i64)
        }
    }

    let serializer = Serializer;
    let result = serializer.serialize_i8(0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::from(0));
}

#[should_panic]
#[test]
fn test_serialize_i8_out_of_bounds() {
    struct Serializer;

    impl Serializer {
        fn serialize_i64(&self, value: i64) -> Result<Value> {
            if value > i64::MAX || value < i64::MIN {
                panic!("Value out of bounds for i64");
            }
            Ok(Value::from(value))
        }

        fn serialize_i8(self, value: i8) -> Result<Value> {
            self.serialize_i64(value as i64)
        }
    }

    let serializer = Serializer;
    let _ = serializer.serialize_i8(128); // This should panic
}

