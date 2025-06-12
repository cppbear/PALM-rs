// Answer 0

#[test]
fn test_serialize_i16_positive() {
    struct Serializer;

    impl Serializer {
        fn serialize_i64(self, value: i64) -> Result<serde_json::Value, serde_json::Error> {
            Ok(serde_json::Value::Number(serde_json::Number::from(value)))
        }

        fn serialize_i16(self, value: i16) -> Result<serde_json::Value, serde_json::Error> {
            self.serialize_i64(value as i64)
        }
    }

    let serializer = Serializer;
    let result = serializer.serialize_i16(1000);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), serde_json::Value::Number(serde_json::Number::from(1000)));
}

#[test]
fn test_serialize_i16_negative() {
    struct Serializer;

    impl Serializer {
        fn serialize_i64(self, value: i64) -> Result<serde_json::Value, serde_json::Error> {
            Ok(serde_json::Value::Number(serde_json::Number::from(value)))
        }

        fn serialize_i16(self, value: i16) -> Result<serde_json::Value, serde_json::Error> {
            self.serialize_i64(value as i64)
        }
    }

    let serializer = Serializer;
    let result = serializer.serialize_i16(-1000);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), serde_json::Value::Number(serde_json::Number::from(-1000)));
}

#[test]
fn test_serialize_i16_zero() {
    struct Serializer;

    impl Serializer {
        fn serialize_i64(self, value: i64) -> Result<serde_json::Value, serde_json::Error> {
            Ok(serde_json::Value::Number(serde_json::Number::from(value)))
        }

        fn serialize_i16(self, value: i16) -> Result<serde_json::Value, serde_json::Error> {
            self.serialize_i64(value as i64)
        }
    }

    let serializer = Serializer;
    let result = serializer.serialize_i16(0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), serde_json::Value::Number(serde_json::Number::from(0)));
}

#[test]
fn test_serialize_i16_boundary_positive() {
    struct Serializer;

    impl Serializer {
        fn serialize_i64(self, value: i64) -> Result<serde_json::Value, serde_json::Error> {
            Ok(serde_json::Value::Number(serde_json::Number::from(value)))
        }

        fn serialize_i16(self, value: i16) -> Result<serde_json::Value, serde_json::Error> {
            self.serialize_i64(value as i64)
        }
    }

    let serializer = Serializer;
    let result = serializer.serialize_i16(i16::MAX);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), serde_json::Value::Number(serde_json::Number::from(i16::MAX as i64)));
}

#[test]
fn test_serialize_i16_boundary_negative() {
    struct Serializer;

    impl Serializer {
        fn serialize_i64(self, value: i64) -> Result<serde_json::Value, serde_json::Error> {
            Ok(serde_json::Value::Number(serde_json::Number::from(value)))
        }

        fn serialize_i16(self, value: i16) -> Result<serde_json::Value, serde_json::Error> {
            self.serialize_i64(value as i64)
        }
    }

    let serializer = Serializer;
    let result = serializer.serialize_i16(i16::MIN);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), serde_json::Value::Number(serde_json::Number::from(i16::MIN as i64)));
}

