// Answer 0

#[test]
fn test_as_object_mut_with_valid_object() {
    let mut v = Value::Object(Map {
        map: MapImpl::new(vec![("key".to_string(), Value::Bool(true))]),
    });
    
    if let Some(map) = v.as_object_mut() {
        map.map.insert("new_key".to_string(), Value::Null);
        assert_eq!(map.map.len(), 2);
    } else {
        panic!("Expected Some(map) but got None");
    }
}

#[test]
fn test_as_object_mut_with_empty_object() {
    let mut v = Value::Object(Map {
        map: MapImpl::new(vec![]),
    });
    
    if let Some(map) = v.as_object_mut() {
        assert_eq!(map.map.len(), 0);
        map.map.insert("another_key".to_string(), Value::Number(Number { n: 42 }));
        assert_eq!(map.map.len(), 1);
    } else {
        panic!("Expected Some(map) but got None");
    }
}

#[test]
fn test_as_object_mut_with_nested_object() {
    let mut v = Value::Object(Map {
        map: MapImpl::new(vec![("outer_key".to_string(), Value::Object(Map {
            map: MapImpl::new(vec![("inner_key".to_string(), Value::Bool(false))]),
        }))]),
    });
    
    if let Some(map) = v.as_object_mut() {
        if let Value::Object(inner_map) = &mut map.map["outer_key"] {
            inner_map.map.insert("new_inner_key".to_string(), Value::String("value".to_string()));
            assert_eq!(inner_map.map.len(), 2);
        } else {
            panic!("Expected Value::Object but got something else");
        }
    } else {
        panic!("Expected Some(map) but got None");
    }
}

#[test]
fn test_as_object_mut_with_non_object_value() {
    let mut v = Value::Bool(false);
    
    assert!(v.as_object_mut().is_none());
}

#[test]
fn test_as_object_mut_with_null_value() {
    let mut v = Value::Null;
    
    assert!(v.as_object_mut().is_none());
}

