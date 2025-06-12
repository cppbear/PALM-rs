// Answer 0

#[test]
fn test_serialize_i128_with_arbitrary_precision() {
    #[cfg(feature = "arbitrary_precision")]
    {
        let value = 123456789012345678901234567890i128;
        let result = serialize_i128(value);
        assert!(result.is_ok());
        if let Ok(v) = result {
            assert_eq!(v, Value::Number(value.into()));
        }
    }
}

#[test]
fn test_serialize_i128_as_u64() {
    let value: i128 = 42;
    let result = serialize_i128(value);
    assert!(result.is_ok());
    if let Ok(v) = result {
        assert_eq!(v, Value::Number(value.try_into().unwrap().into()));
    }
}

#[test]
fn test_serialize_i128_as_i64() {
    let value: i128 = -42;
    let result = serialize_i128(value);
    assert!(result.is_ok());
    if let Ok(v) = result {
        assert_eq!(v, Value::Number(value.try_into().unwrap().into()));
    }
}

#[test]
#[should_panic]
fn test_serialize_i128_out_of_range() {
    let value: i128 = 1i128 << 70; // This exceeds u64
    let result = serialize_i128(value);
    assert!(result.is_err());
}

