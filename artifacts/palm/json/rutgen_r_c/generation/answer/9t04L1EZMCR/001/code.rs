// Answer 0

#[test]
fn test_pointer_mut_empty_pointer() {
    let mut value = Value::Null; // Using the simplest Value to test the empty pointer case
    assert_eq!(value.pointer_mut(""), Some(&mut value));
}

#[test]
fn test_pointer_mut_invalid_pointer() {
    let mut value = Value::Object(Map { map: MapImpl::new() }); // Initializing an empty Object
    assert_eq!(value.pointer_mut("invalid_pointer"), None);
}

#[test]
fn test_pointer_mut_pointer_to_nonexistent_key() {
    let mut value = Value::Object(Map {
        map: MapImpl::new(),
    });
    assert_eq!(value.pointer_mut("/nonexistent_key"), None);
}

#[test]
fn test_pointer_mut_pointer_to_existing_key() {
    let mut value = Value::Object(Map {
        map: {
            let mut m = MapImpl::new();
            m.insert("key".to_string(), Value::Number(Number { n: 1 }));
            m
        },
    });
    if let Some(existing_value) = value.pointer_mut("/key") {
        assert_eq!(existing_value.as_number().unwrap().n, 1);
        *existing_value = Value::Number(Number { n: 2 }); // Mutate the value
    }
    assert_eq!(value.pointer("/key").unwrap().as_number().unwrap().n, 2);
}

