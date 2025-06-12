// Answer 0

#[test]
fn test_index_mut_with_object_insert_new_key() {
    let mut data = Value::Object(Map { map: MapImpl::new() });
    data["new_key"] = Value::Null;
    if let Value::Object(map) = data {
        assert!(map.map.contains_key("new_key"));
    }
}

#[test]
fn test_index_mut_with_object_replace_existing_key() {
    let mut data = Value::Object(Map { map: MapImpl::new() });
    data["existing_key"] = Value::Int(5);
    data["existing_key"] = Value::Int(10);
    if let Value::Object(map) = data {
        assert_eq!(map.map.get("existing_key"), Some(&Value::Int(10)));
    }
}

#[test]
#[should_panic]
fn test_index_mut_with_non_object_panic() {
    let mut data = Value::Int(42);
    let _ = data["key"]; // This should panic
}

#[test]
fn test_index_mut_with_array_insert() {
    let mut data = Value::Array(vec![Value::Null; 3]);
    data[1] = Value::Bool(true);
    if let Value::Array(arr) = data {
        assert_eq!(arr[1], Value::Bool(true));
    }
}

#[test]
#[should_panic]
fn test_index_mut_with_array_too_small() {
    let mut data = Value::Array(vec![Value::Null]);
    let _ = data[1]; // This should panic due to index being out of bounds
}

#[test]
#[should_panic]
fn test_index_mut_with_null_as_object_panic() {
    let mut data = Value::Null;
    let _ = data["key"]; // This should panic
}

#[test]
fn test_index_mut_with_deep_nested_insertion() {
    let mut data = Value::Object(Map { map: MapImpl::new() });
    data["a"]["b"]["c"] = Value::Bool(true);
    if let Value::Object(map_a) = data {
        if let Value::Object(map_b) = map_a.map.get("a") {
            if let Value::Object(map_c) = map_b.map.get("b") {
                assert_eq!(map_c.map.get("c"), Some(&Value::Bool(true)));
            }
        }
    }
}

