// Answer 0

#[test]
fn test_deserialize_i64() {
    use serde_json::Deserializer;

    let json_value: i64 = 42;
    let de = Deserializer::from_value(serde_json::json!(json_value));
    let result: Result<Number, _> = deserialize(de);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Number::from(42));
}

#[test]
fn test_deserialize_u64() {
    use serde_json::Deserializer;

    let json_value: u64 = 42;
    let de = Deserializer::from_value(serde_json::json!(json_value));
    let result: Result<Number, _> = deserialize(de);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Number::from(42));
}

#[test]
fn test_deserialize_i128() {
    use serde_json::Deserializer;

    let json_value: i128 = -123456789012345678901234567890;
    let de = Deserializer::from_value(serde_json::json!(json_value));
    let result: Result<Number, _> = deserialize(de);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Number::from_i128(json_value).unwrap());
}

#[test]
fn test_deserialize_u128() {
    use serde_json::Deserializer;

    let json_value: u128 = 123456789012345678901234567890;
    let de = Deserializer::from_value(serde_json::json!(json_value));
    let result: Result<Number, _> = deserialize(de);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Number::from_u128(json_value).unwrap());
}

#[test]
fn test_deserialize_f64() {
    use serde_json::Deserializer;

    let json_value: f64 = 3.14;
    let de = Deserializer::from_value(serde_json::json!(json_value));
    let result: Result<Number, _> = deserialize(de);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Number::from_f64(json_value).unwrap());
}

#[test]
#[should_panic]
fn test_deserialize_invalid_i128() {
    use serde_json::Deserializer;

    let json_value: i128 = 123456789012345678901234567890; // out of range for i128
    let de = Deserializer::from_value(serde_json::json!(json_value));
    let _: Result<Number, _> = deserialize(de).unwrap();
}

#[test]
#[should_panic]
fn test_deserialize_invalid_u128() {
    use serde_json::Deserializer;

    let json_value: u128 = 123456789012345678901234567890; // out of range for u128
    let de = Deserializer::from_value(serde_json::json!(json_value));
    let _: Result<Number, _> = deserialize(de).unwrap();
}

#[test]
#[should_panic]
fn test_deserialize_invalid_f64() {
    use serde_json::Deserializer;

    let json_value: f64 = f64::NAN; // invalid floating-point number
    let de = Deserializer::from_value(serde_json::json!(json_value));
    let _: Result<Number, _> = deserialize(de).unwrap();
}

