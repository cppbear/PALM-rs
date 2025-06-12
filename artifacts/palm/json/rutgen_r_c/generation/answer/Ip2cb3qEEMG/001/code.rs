// Answer 0

#[test]
fn test_serialize_u128_within_u64_range() {
    let serializer = Serializer;
    let value: u128 = 100; // within u64 range
    let result = serializer.serialize_u128(value);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_u128_out_of_u64_range() {
    let serializer = Serializer;
    let value: u128 = u128::MAX; // out of u64 range
    let result = serializer.serialize_u128(value);
    assert!(result.is_err());
    
    if let Err(err) = result {
        match err.err.as_ref() {
            ErrorImpl { code: ErrorCode::NumberOutOfRange, line: 0, column: 0 } => {},
            _ => panic!("Unexpected error type: {:?}", err),
        }
    }
}

