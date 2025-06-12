// Answer 0

#[test]
fn test_end_with_map() {
    let mut map = Map { map: std::collections::BTreeMap::new() };
    map.map.insert("key1".to_owned(), Value::String("value1".to_owned()));
    map.map.insert("key2".to_owned(), Value::Bool(true));

    let serialize_map = SerializeMap::Map { map, next_key: None };
    let result = serialize_map.end();

    assert!(result.is_ok());
    if let Ok(value) = result {
        match value {
            Value::Object(obj) => {
                assert_eq!(obj.map.len(), 2);
                assert_eq!(obj.map.get("key1").unwrap(), &Value::String("value1".to_owned()));
                assert_eq!(obj.map.get("key2").unwrap(), &Value::Bool(true));
            },
            _ => panic!("Expected a Value::Object"),
        }
    }
}

#[test]
#[should_panic]
fn test_end_with_arbitrary_precision() {
    #[cfg(feature = "arbitrary_precision")]
    {
        let serialize_map = SerializeMap::Number { out_value: None };
        serialize_map.end();
    }
}

#[test]
#[should_panic]
fn test_end_with_raw_value() {
    #[cfg(feature = "raw_value")]
    {
        let serialize_map = SerializeMap::RawValue { out_value: None };
        serialize_map.end();
    }
}

