// Answer 0

#[test]
fn test_serialize_i128_negative_value_large() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_i128(self, value: i128) -> Result<Value> {
            #[cfg(feature = "arbitrary_precision")]
            {
                Ok(Value::Number(value.into()))
            }

            #[cfg(not(feature = "arbitrary_precision"))]
            {
                if let Ok(value) = u64::try_from(value) {
                    Ok(Value::Number(value.into()))
                } else if let Ok(value) = i64::try_from(value) {
                    Ok(Value::Number(value.into()))
                } else {
                    Err(Error::syntax(ErrorCode::NumberOutOfRange, 0, 0))
                }
            }
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_i128(-1i128 << 64); // A large negative value
    assert!(result.is_err());
}

#[test]
fn test_serialize_i128_max_i64() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_i128(self, value: i128) -> Result<Value> {
            #[cfg(feature = "arbitrary_precision")]
            {
                Ok(Value::Number(value.into()))
            }

            #[cfg(not(feature = "arbitrary_precision"))]
            {
                if let Ok(value) = u64::try_from(value) {
                    Ok(Value::Number(value.into()))
                } else if let Ok(value) = i64::try_from(value) {
                    Ok(Value::Number(value.into()))
                } else {
                    Err(Error::syntax(ErrorCode::NumberOutOfRange, 0, 0))
                }
            }
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_i128(i64::MAX as i128); // Should be valid
    assert!(result.is_ok());
}

#[test]
fn test_serialize_i128_min_i64() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_i128(self, value: i128) -> Result<Value> {
            #[cfg(feature = "arbitrary_precision")]
            {
                Ok(Value::Number(value.into()))
            }

            #[cfg(not(feature = "arbitrary_precision"))]
            {
                if let Ok(value) = u64::try_from(value) {
                    Ok(Value::Number(value.into()))
                } else if let Ok(value) = i64::try_from(value) {
                    Ok(Value::Number(value.into()))
                } else {
                    Err(Error::syntax(ErrorCode::NumberOutOfRange, 0, 0))
                }
            }
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_i128(i64::MIN as i128); // Should be valid
    assert!(result.is_ok());
}

