// Answer 0

#[test]
fn test_end_with_map() {
    struct SerializeMap {
        // Dummy field for our map variant
        is_map: bool,
    }

    impl SerializeMap {
        fn end(self) -> Result<Value> {
            match self {
                SerializeMap { is_map: true } => Ok(Value::Object(serde_json::Map::new())),
                _ => Err("Not a valid map".into()),
            }
        }
    }

    let map_variant = SerializeMap { is_map: true };
    let result = map_variant.end();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Object(serde_json::Map::new()));
}

#[cfg(feature = "arbitrary_precision")]
#[test]
fn test_end_with_number() {
    struct SerializeMap {
        out_value: Option<Value>,
    }

    impl SerializeMap {
        fn end(self) -> Result<Value> {
            match self {
                SerializeMap { out_value: Some(value) } => Ok(value),
                _ => Err("number value was not emitted".into()),
            }
        }
    }

    let number_variant = SerializeMap { out_value: Some(Value::Number(serde_json::Number::from(42))) };
    let result = number_variant.end();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Number(serde_json::Number::from(42)));
}

#[cfg(feature = "raw_value")]
#[test]
fn test_end_with_raw_value() {
    struct SerializeMap {
        out_value: Option<Value>,
    }

    impl SerializeMap {
        fn end(self) -> Result<Value> {
            match self {
                SerializeMap { out_value: Some(value) } => Ok(value),
                _ => Err("raw value was not emitted".into()),
            }
        }
    }

    let raw_variant = SerializeMap { out_value: Some(Value::String("raw".to_string())) };
    let result = raw_variant.end();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::String("raw".to_string()));
}

#[cfg(feature = "arbitrary_precision")]
#[test]
#[should_panic(expected = "number value was not emitted")]
fn test_end_number_panics() {
    struct SerializeMap {
        out_value: Option<Value>,
    }

    impl SerializeMap {
        fn end(self) -> Result<Value> {
            match self {
                SerializeMap { out_value: Some(value) } => Ok(value),
                _ => panic!("number value was not emitted"),
            }
        }
    }

    let number_variant = SerializeMap { out_value: None };
    let _ = number_variant.end();
}

#[cfg(feature = "raw_value")]
#[test]
#[should_panic(expected = "raw value was not emitted")]
fn test_end_raw_value_panics() {
    struct SerializeMap {
        out_value: Option<Value>,
    }

    impl SerializeMap {
        fn end(self) -> Result<Value> {
            match self {
                SerializeMap { out_value: Some(value) } => Ok(value),
                _ => panic!("raw value was not emitted"),
            }
        }
    }

    let raw_variant = SerializeMap { out_value: None };
    let _ = raw_variant.end();
}

