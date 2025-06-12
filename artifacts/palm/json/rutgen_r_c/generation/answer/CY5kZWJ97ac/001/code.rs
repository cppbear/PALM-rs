// Answer 0

#[test]
fn test_sort_all_objects_with_empty_object() {
    let mut value = Value::Object(Map { map: MapImpl::new() });
    value.sort_all_objects();
    assert!(value.is_object());
}

#[test]
fn test_sort_all_objects_with_single_entry_object() {
    let mut value = Value::Object(Map { map: MapImpl::new(vec![("b".to_string(), Value::string("value_b"))]) });
    value.sort_all_objects();
    assert!(value.is_object());
    if let Value::Object(map) = value {
        assert_eq!(map.len(), 1);
        assert!(map.contains_key("b"));
    }
}

#[test]
fn test_sort_all_objects_with_unordered_entries() {
    let mut value = Value::Object(Map { map: MapImpl::new(vec![
        ("b".to_string(), Value::string("value_b")),
        ("a".to_string(), Value::string("value_a")),
    ]) });
    value.sort_all_objects();
    assert!(value.is_object());
    if let Value::Object(map) = value {
        let keys: Vec<_> = map.keys().collect();
        assert_eq!(keys, vec!["a", "b"]); // Check if sorted correctly
    }
}

#[test]
fn test_sort_all_objects_with_nesting() {
    let mut value = Value::Array(vec![
        Value::Object(Map { map: MapImpl::new(vec![("c".to_string(), Value::string("value_c"))]) }),
        Value::Object(Map { map: MapImpl::new(vec![("a".to_string(), Value::string("value_a"))]) }),
    ]);
    value.sort_all_objects();
    assert!(value.is_array());
    if let Value::Array(array) = value {
        assert_eq!(array.len(), 2);
        assert!(array[0].is_object());
        assert!(array[1].is_object());
        if let Value::Object(map_a) = &array[0] {
            assert_eq!(map_a.keys().collect::<Vec<_>>(), vec!["a"]); // First object should be "a"
        }
        if let Value::Object(map_c) = &array[1] {
            assert_eq!(map_c.keys().collect::<Vec<_>>(), vec!["c"]); // Second object should be "c"
        }
    }
}

#[test]
fn test_sort_all_objects_with_complex_nesting() {
    let mut value = Value::Object(Map { map: MapImpl::new(vec![
        ("b".to_string(), Value::Object(Map { map: MapImpl::new(vec![("b2".to_string(), Value::string("value_b2"))]) })),
        ("a".to_string(), Value::Array(vec![Value::string("value_a1"), Value::string("value_a2")])),
    ]) });
    value.sort_all_objects();
    assert!(value.is_object());
}

