// Answer 0

#[test]
fn test_pointer_mut_empty_pointer() {
    let mut value = Value::Null;
    assert_eq!(value.pointer_mut(""), Some(&mut value));
}

#[test]
fn test_pointer_mut_invalid_pointer() {
    let mut value = Value::Object(Map { map: MapImpl::new() });
    assert_eq!(value.pointer_mut("invalid_pointer"), None);
}

#[test]
fn test_pointer_mut_from_object() {
    let mut value = Value::Object(Map {
        map: MapImpl::from(vec![("key".to_string(), Value::Number(Number { n: 1 }))]),
    });
    if let Some(v) = value.pointer_mut("/key") {
        *v = Value::Number(Number { n: 2 });
    }
    assert_eq!(value.pointer("/key"), Some(&Value::Number(Number { n: 2 })));
}

#[test]
fn test_pointer_mut_from_array() {
    let mut value = Value::Array(vec![Value::Number(Number { n: 1 }), Value::Number(Number { n: 2 })]);
    if let Some(v) = value.pointer_mut("/0") {
        *v = Value::Number(Number { n: 3 });
    }
    assert_eq!(value.pointer("/0"), Some(&Value::Number(Number { n: 3 })));
}

#[test]
fn test_pointer_mut_nested_object() {
    let mut value = Value::Object(Map {
        map: MapImpl::from(vec![
            ("outer".to_string(), Value::Object(Map {
                map: MapImpl::from(vec![("inner".to_string(), Value::Number(Number { n: 42 }))]),
            })),
        ]),
    });
    if let Some(v) = value.pointer_mut("/outer/inner") {
        *v = Value::Number(Number { n: 99 });
    }
    assert_eq!(value.pointer("/outer/inner"), Some(&Value::Number(Number { n: 99 })));
}

