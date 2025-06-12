// Answer 0

#[test]
fn test_serialize_u128_valid_u64() {
    let value: u128 = 12345678901234567890; // This is within u64 range
    let result = serialize_u128(value);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_u128_out_of_range() {
    let value: u128 = u128::MAX; // This exceeds u64 range
    let result = serialize_u128(value);
    assert!(result.is_err());
    if let Err(e) = result {
        match e {
            Error::syntax(ErrorCode::NumberOutOfRange, 0, 0) => {}
            _ => panic!("Unexpected error: {:?}", e),
        }
    }
}

