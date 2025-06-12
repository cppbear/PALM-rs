// Answer 0

#[test]
fn test_sort_all_objects_empty_object() {
    let mut value = Value::Object(Map { map: MapImpl::new() });
    value.sort_all_objects();
    assert!(value.is_object());
}

#[test]
fn test_sort_all_objects_single_entry_object() {
    let mut value = Value::Object(Map {
        map: MapImpl::from(vec![("b".to_string(), Value::String("value".to_string()))]),
    });
    value.sort_all_objects();
    assert!(value.is_object());
    let obj = value.as_object().unwrap();
    assert_eq!(obj.len(), 1);
}

#[test]
fn test_sort_all_objects_multiple_entries_object() {
    let mut value = Value::Object(Map {
        map: MapImpl::from(vec![
            ("b".to_string(), Value::String("value".to_string())),
            ("a".to_string(), Value::String("value".to_string())),
        ]),
    });
    value.sort_all_objects();
    let obj = value.as_object().unwrap();
    assert_eq!(obj.keys().collect::<Vec<&String>>(), vec![&"a".to_string(), &"b".to_string()]);
}

#[test]
fn test_sort_all_objects_nested_objects() {
    let mut value = Value::Object(Map {
        map: MapImpl::from(vec![
            (
                "b".to_string(),
                Value::Object(Map {
                    map: MapImpl::from(vec![
                        ("c".to_string(), Value::String("value_c".to_string())),
                        ("a".to_string(), Value::String("value_a".to_string())),
                    ]),
                }),
            ),
            (
                "a".to_string(),
                Value::String("value_a".to_string()),
            ),
        ]),
    });
    value.sort_all_objects();
    let obj = value.as_object().unwrap();
    let nested_obj = if let Value::Object(ref nested_map) = obj["b"] {
        nested_map
    } else { panic!("Expected nested object") };
    
    assert_eq!(obj.keys().collect::<Vec<&String>>(), vec![&"a".to_string(), &"b".to_string()]);
    assert_eq!(nested_obj.keys().collect::<Vec<&String>>(), vec![&"a".to_string(), &"c".to_string()]);
}

#[test]
#[should_panic]
fn test_sort_all_objects_non_object() {
    let mut value = Value::String("not an object".to_string());
    value.sort_all_objects(); // should not panic, but no effect
    assert!(!value.is_object());
}

