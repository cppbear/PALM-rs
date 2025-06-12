// Answer 0

#[test]
fn test_deserialize_i64() {
    use serde_json::de::Deserializer;
    use serde_json::Value;

    let json_value = Value::from(42i64);
    let deserializer = Deserializer::from_value(json_value);
    let result: Result<Number, _> = deserialize(deserializer);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Number::from(42));
}

#[test]
fn test_deserialize_i128() {
    use serde_json::de::Deserializer;
    use serde_json::Value;

    let json_value = Value::from(123456789012345678901234567890i128);
    let deserializer = Deserializer::from_value(json_value);
    let result: Result<Number, _> = deserialize(deserializer);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Number::from_i128(123456789012345678901234567890).unwrap());
}

#[test]
fn test_deserialize_u64() {
    use serde_json::de::Deserializer;
    use serde_json::Value;

    let json_value = Value::from(42u64);
    let deserializer = Deserializer::from_value(json_value);
    let result: Result<Number, _> = deserialize(deserializer);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Number::from(42));
}

#[test]
fn test_deserialize_u128() {
    use serde_json::de::Deserializer;
    use serde_json::Value;

    let json_value = Value::from(123456789012345678901234567890u128);
    let deserializer = Deserializer::from_value(json_value);
    let result: Result<Number, _> = deserialize(deserializer);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Number::from_u128(123456789012345678901234567890).unwrap());
}

#[test]
fn test_deserialize_f64() {
    use serde_json::de::Deserializer;
    use serde_json::Value;

    let json_value = Value::from(3.14f64);
    let deserializer = Deserializer::from_value(json_value);
    let result: Result<Number, _> = deserialize(deserializer);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Number::from_f64(3.14).unwrap());
}

#[test]
#[should_panic]
fn test_deserialize_invalid_type() {
    use serde_json::de::Deserializer;
    use serde_json::Value;

    let json_value = Value::from("not a number");
    let deserializer = Deserializer::from_value(json_value);
    let _result: Result<Number, _> = deserialize(deserializer).unwrap();
}

#[test]
fn test_deserialize_arbitrary_precision() {
    #[cfg(feature = "arbitrary_precision")]
    {
        use serde_json::de::Deserializer;
        use serde_json::Value;

        let json_value = Value::Object(serde_json::Map::new());
        let deserializer = Deserializer::from_value(json_value);
        let result: Result<Number, _> = deserialize(deserializer);

        assert!(result.is_err());
    }
}

