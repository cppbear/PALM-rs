// Answer 0

#[test]
fn test_serialize_u128_with_arbitrary_precision() {
    let serializer = Serializer;
    let result = serializer.serialize_u128(123456789012345678901234567890u128);
    #[cfg(feature = "arbitrary_precision")]
    {
        assert!(result.is_ok());
        let value = result.unwrap();
        match value {
            Value::Number(num) => {
                // In arbitrary precision, the number should be serialized correctly
                assert_eq!(num.n, 123456789012345678901234567890u128.into());
            },
            _ => panic!("Expected Value::Number"),
        }
    }
}

#[test]
fn test_serialize_u128_without_arbitrary_precision() {
    let serializer = Serializer;
    let result = serializer.serialize_u128(123456789012345678901234567890u128);
    #[cfg(not(feature = "arbitrary_precision"))]
    {
        assert!(result.is_err());
        let error = result.unwrap_err();
        match error.err.code {
            ErrorCode::NumberOutOfRange => {},
            _ => panic!("Expected NumberOutOfRange error"),
        }
    }
}

#[test]
fn test_serialize_u128_within_u64_range() {
    let serializer = Serializer;
    let result = serializer.serialize_u128(12345678901234567890u128);
    #[cfg(not(feature = "arbitrary_precision"))]
    {
        assert!(result.is_ok());
        let value = result.unwrap();
        match value {
            Value::Number(num) => {
                assert_eq!(num.n, 12345678901234567890u128.into());
            },
            _ => panic!("Expected Value::Number"),
        }
    }
}

