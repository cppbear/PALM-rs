// Answer 0

#[test]
fn test_serialize_u64_success() {
    let serializer = Serializer;
    let result = serializer.serialize_u64(0u64);
    assert!(result.is_ok());
    if let Ok(value) = result {
        match value {
            Value::Number(num) => {
                assert_eq!(num.n, 0.into()); // Assuming N can be converted from u64 directly
            }
            _ => panic!("Expected Value::Number"),
        }
    }
}

#[test]
fn test_serialize_u64_large_value() {
    let serializer = Serializer;
    let large_value: u64 = 1_000_000_000_000_000_000;
    let result = serializer.serialize_u64(large_value);
    assert!(result.is_ok());
    if let Ok(value) = result {
        match value {
            Value::Number(num) => {
                assert_eq!(num.n, large_value.into()); // Assuming N can be converted from u64 directly
            }
            _ => panic!("Expected Value::Number"),
        }
    }
}

#[test]
fn test_serialize_u64_boundary_value() {
    let serializer = Serializer;
    let boundary_value: u64 = std::u64::MAX;
    let result = serializer.serialize_u64(boundary_value);
    assert!(result.is_ok());
    if let Ok(value) = result {
        match value {
            Value::Number(num) => {
                assert_eq!(num.n, boundary_value.into()); // Assuming N can be converted from u64 directly
            }
            _ => panic!("Expected Value::Number"),
        }
    }
}

