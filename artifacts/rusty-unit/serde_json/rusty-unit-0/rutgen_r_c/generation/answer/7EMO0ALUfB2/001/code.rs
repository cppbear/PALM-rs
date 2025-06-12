// Answer 0

#[test]
fn test_serialize_value_without_key_should_panic() {
    let mut serialize_map = SerializeMap::Map { 
        map: Map::new(), 
        next_key: None 
    };

    let result = std::panic::catch_unwind(|| {
        serialize_map.serialize_value(&Value::Bool(true)).unwrap();
    });

    assert!(result.is_err());
}

#[test]
fn test_serialize_value_with_invalid_to_value() {
    struct InvalidSerializable;

    impl Serialize for InvalidSerializable {
        fn serialize<S>(&self, _: S) -> Result<(), S::Error>
        where
            S: serde::ser::Serializer,
        {
            Err(ser::Error::custom("invalid serialization"))
        }
    }

    let mut serialize_map = SerializeMap::Map { 
        map: Map::new(), 
        next_key: Some("key1".to_string())
    };

    let result = serialize_map.serialize_value(&InvalidSerializable);

    assert!(result.is_err());
}

