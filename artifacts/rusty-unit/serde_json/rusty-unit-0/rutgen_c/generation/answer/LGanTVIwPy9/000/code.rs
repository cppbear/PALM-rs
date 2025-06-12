// Answer 0

#[test]
fn test_end_with_map() {
    struct TestSerializeMap {
        map: Map<String, Value>,
        next_key: Option<String>,
    }

    let map = Map { map: MapImpl::<String, Value>::new() };
    let serialize_map = SerializeMap::Map { map, next_key: None };

    let result = serialize_map.end();
    assert!(result.is_ok());
}

#[test]
fn test_end_with_arbitrary_precision() {
    #[cfg(feature = "arbitrary_precision")]
    {
        struct TestSerializeMap {
            out_value: Option<Value>,
        }

        let serialize_map = SerializeMap::Number { out_value: Some(Value::Number(12.5.into())) };

        let result = serialize_map.end();
        assert_eq!(result.unwrap(), Value::Number(12.5.into()));
    }
}

#[test]
fn test_end_with_raw_value() {
    #[cfg(feature = "raw_value")]
    {
        struct TestSerializeMap {
            out_value: Option<Value>,
        }

        let serialize_map = SerializeMap::RawValue { out_value: Some(Value::String("raw_value".to_string())) };

        let result = serialize_map.end();
        assert_eq!(result.unwrap(), Value::String("raw_value".to_string()));
    }
}

#[should_panic(expected = "number value was not emitted")]
#[cfg(feature = "arbitrary_precision")]
#[test]
fn test_end_number_with_none() {
    let serialize_map = SerializeMap::Number { out_value: None };
    let _ = serialize_map.end();
}

#[should_panic(expected = "raw value was not emitted")]
#[cfg(feature = "raw_value")]
#[test]
fn test_end_raw_value_with_none() {
    let serialize_map = SerializeMap::RawValue { out_value: None };
    let _ = serialize_map.end();
}

