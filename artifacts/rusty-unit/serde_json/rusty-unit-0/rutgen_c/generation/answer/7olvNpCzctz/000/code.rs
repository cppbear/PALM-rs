// Answer 0

#[test]
fn test_serialize_f32() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(3.14);
    assert!(result.is_ok());
    if let Ok(value) = result {
        match value {
            Value::Number(num) => {
                // Assuming a method to get f32 from Value::Number exists
                assert_eq!(num.to_f32(), Some(3.14));
            }
            _ => panic!("Expected Value::Number"),
        }
    }
}

#[test]
fn test_serialize_f32_zero() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(0.0);
    assert!(result.is_ok());
    if let Ok(value) = result {
        match value {
            Value::Number(num) => {
                assert_eq!(num.to_f32(), Some(0.0));
            }
            _ => panic!("Expected Value::Number"),
        }
    }
}

#[test]
fn test_serialize_f32_negative() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(-2.71);
    assert!(result.is_ok());
    if let Ok(value) = result {
        match value {
            Value::Number(num) => {
                assert_eq!(num.to_f32(), Some(-2.71));
            }
            _ => panic!("Expected Value::Number"),
        }
    }
}

