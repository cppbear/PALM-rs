// Answer 0

#[test]
fn test_serialize_i128_with_arbitrary_precision() {
    #[cfg(feature = "arbitrary_precision")]
    {
        let serializer = Serializer;
        let value: i128 = 123456789012345678901234567890i128;
        let result = serializer.serialize_i128(value).unwrap();
        if let Value::Number(num) = result {
            assert_eq!(num.n, value.into());
        } else {
            panic!("Expected Value::Number");
        }
    }
}

#[test]
fn test_serialize_i128_within_u64_range() {
    let serializer = Serializer;
    let value: i128 = 1234567890;
    let result = serializer.serialize_i128(value).unwrap();
    
    if let Value::Number(num) = result {
        assert_eq!(num.n, value.into());
    } else {
        panic!("Expected Value::Number");
    }
}

#[test]
fn test_serialize_i128_within_i64_range() {
    let serializer = Serializer;
    let value: i128 = -1234567890; // Within the range of i64
    let result = serializer.serialize_i128(value).unwrap();

    if let Value::Number(num) = result {
        assert_eq!(num.n, value.into());
    } else {
        panic!("Expected Value::Number");
    }
}

#[test]
fn test_serialize_i128_out_of_range() {
    let serializer = Serializer;
    let value: i128 = i128::MAX; // Out of range for u64
    let result = serializer.serialize_i128(value);
    
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.err.code, ErrorCode::NumberOutOfRange);
    }
}

#[test]
fn test_serialize_i128_negative_out_of_range() {
    let serializer = Serializer;
    let value: i128 = i128::MIN; // Out of range for i64
    let result = serializer.serialize_i128(value);
    
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.err.code, ErrorCode::NumberOutOfRange);
    }
}

