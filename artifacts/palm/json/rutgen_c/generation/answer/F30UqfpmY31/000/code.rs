// Answer 0

#[test]
fn test_index_mut_for_null() {
    let mut value = Value::Null;
    value["key"] = Value::Bool(true);
    match value {
        Value::Object(ref map) => {
            assert_eq!(map.map.len(), 1);
            assert!(map.map.contains_key("key"));
        }
        _ => panic!("Expected an Object variant."),
    }
}

#[test]
fn test_index_mut_for_object() {
    let mut value = Value::Object(Map { map: MapImpl::new() });
    value["key"] = Value::Bool(true);
    match value {
        Value::Object(ref map) => {
            assert_eq!(map.map.len(), 1);
            assert!(map.map.contains_key("key"));
        }
        _ => panic!("Expected an Object variant."),
    }
}

#[test]
#[should_panic]
fn test_index_mut_for_non_object() {
    let mut value = Value::Bool(true);
    value["key"] = Value::Bool(false); // Should panic
}

#[test]
fn test_index_mut_for_array() {
    let mut value = Value::Array(vec![Value::Bool(false); 3]);
    value[0] = Value::Bool(true);
    match value {
        Value::Array(ref arr) => {
            assert_eq!(arr.len(), 3);
            assert_eq!(arr[0], Value::Bool(true));
        }
        _ => panic!("Expected an Array variant."),
    }
}

#[test]
#[should_panic]
fn test_index_mut_array_too_small() {
    let mut value = Value::Array(vec![Value::Bool(false); 2]);
    value[3] = Value::Bool(true); // Should panic
}

