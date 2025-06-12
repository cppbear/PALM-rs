// Answer 0

#[test]
fn test_into_deserializer_with_single_entry() {
    let map = Map {
        map: MapImpl::new(),
    };
    let _deserializer = map.into_deserializer();
}

#[test]
fn test_into_deserializer_with_multiple_entries() {
    let mut map = Map {
        map: MapImpl::new(),
    };
    map.map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.map.insert("key2".to_string(), Value::Number(Number::from(42)));
    let _deserializer = map.into_deserializer();
}

#[test]
fn test_into_deserializer_with_nested_objects() {
    let mut map = Map {
        map: MapImpl::new(),
    };
    let inner_map = Map {
        map: MapImpl::new(),
    };
    map.map.insert("outer_key".to_string(), Value::Object(inner_map));
    let _deserializer = map.into_deserializer();
}

#[test]
fn test_into_deserializer_with_complex_array() {
    let mut map = Map {
        map: MapImpl::new(),
    };
    let array_value = Value::Array(vec![
        Value::Number(Number::from(1)),
        Value::String("two".to_string()),
        Value::Bool(true),
    ]);
    map.map.insert("array_key".to_string(), array_value);
    let _deserializer = map.into_deserializer();
}

#[test]
fn test_into_deserializer_with_depth_limit() {
    let mut map = Map {
        map: MapImpl::new(),
    };
    let mut current_map = &mut map;
    for i in 0..5 {
        let new_map = Map {
            map: MapImpl::new(),
        };
        current_map.map.insert(format!("key{}", i), Value::Object(new_map));
        current_map = current_map.map.get_mut(&format!("key{}", i)).unwrap().as_object_mut().unwrap();
    }
    let _deserializer = map.into_deserializer();
}

#[test]
fn test_into_deserializer_with_large_map() {
    let mut map = Map {
        map: MapImpl::new(),
    };
    for i in 0..100 {
        map.map.insert(format!("key{}", i), Value::String(format!("value{}", i)));
    }
    let _deserializer = map.into_deserializer();
}

