// Answer 0

#[test]
fn test_serialize_u128_arbitrary_precision() {
    #[cfg(feature = "arbitrary_precision")]
    {
        let value = 123456789012345678901234567890u128;
        let result = serialize_u128(value);
        assert!(result.is_ok());
        if let Ok(v) = result {
            match v {
                Value::Number(n) => assert_eq!(n, 123456789012345678901234567890.into()),
                _ => panic!("Expected Value::Number"),
            }
        }
    }
}

#[test]
fn test_serialize_u128_within_u64_bounds() {
    #[cfg(not(feature = "arbitrary_precision"))]
    {
        let value = 12345678901234567890u128; // Within u64 bounds
        let result = serialize_u128(value);
        assert!(result.is_ok());
        if let Ok(v) = result {
            match v {
                Value::Number(n) => assert_eq!(n, 12345678901234567890u64.into()),
                _ => panic!("Expected Value::Number"),
            }
        }
    }
}

#[test]
#[should_panic]
fn test_serialize_u128_out_of_u64_bounds() {
    #[cfg(not(feature = "arbitrary_precision"))]
    {
        let value = 123456789012345678901234567890u128; // Out of u64 bounds
        let result = serialize_u128(value);
        assert!(result.is_err());
    }
}

