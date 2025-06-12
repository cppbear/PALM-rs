// Answer 0

#[test]
fn test_end_serialize_map() {
    use serde::ser::Serializer;

    // Construct a SerializeMap with a simple empty Map
    let empty_map = Map { map: MapImpl::new() };
    let serialize_map = SerializeMap::Map { map: empty_map, next_key: None };
    
    // Test the end function for the empty map
    let result = serialize_map.end();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Object(Map::new()));

    // Construct a SerializeMap with a populated Map
    let mut populated_map = Map { map: MapImpl::new() };
    // Assume we have some method to insert values in the MapImpl for our test
    populated_map.map.insert("key1".to_string(), Value::String("value1".to_string()));

    let serialize_map_populated = SerializeMap::Map { map: populated_map, next_key: None };
    
    // Test the end function for the populated map
    let result_populated = serialize_map_populated.end();
    assert!(result_populated.is_ok());
    match result_populated.unwrap() {
        Value::Object(result_map) => {
            assert_eq!(result_map.get("key1"), Some(&Value::String("value1".to_string())));
        },
        _ => panic!("Expected Value::Object"),
    }
}

#[test]
#[should_panic(expected = "number value was not emitted")]
fn test_end_arbitrary_precision_panic() {
    #[cfg(feature = "arbitrary_precision")]
    {
        let serialize_map = SerializeMap::Number { out_value: None };
        serialize_map.end();
    }
}

#[test]
#[should_panic(expected = "raw value was not emitted")]
fn test_end_raw_value_panic() {
    #[cfg(feature = "raw_value")]
    {
        let serialize_map = SerializeMap::RawValue { out_value: None };
        serialize_map.end();
    }
}

