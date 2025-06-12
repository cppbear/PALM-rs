// Answer 0

#[test]
fn test_get_on_object_with_existing_key() {
    let object = Value::Object(Map {
        map: vec![
            ("A".to_string(), Value::Number(Number { n: 65 })),
            ("B".to_string(), Value::Number(Number { n: 66 })),
            ("C".to_string(), Value::Number(Number { n: 67 })),
        ].into_iter().collect(),
    });
    assert_eq!(object.get("A"), Some(&Value::Number(Number { n: 65 })));
}

#[test]
fn test_get_on_object_with_non_existing_key() {
    let object = Value::Object(Map {
        map: vec![
            ("A".to_string(), Value::Number(Number { n: 65 })),
        ].into_iter().collect(),
    });
    assert_eq!(object.get("B"), None);
}

#[test]
fn test_get_on_array_with_existing_index() {
    let array = Value::Array(vec![
        Value::String("A".to_string()),
        Value::String("B".to_string()),
        Value::String("C".to_string()),
    ]);
    assert_eq!(array.get(2), Some(&Value::String("C".to_string())));
}

#[test]
fn test_get_on_array_with_out_of_bounds_index() {
    let array = Value::Array(vec![
        Value::String("A".to_string()),
        Value::String("B".to_string()),
    ]);
    assert_eq!(array.get(3), None);
}

#[test]
fn test_get_on_array_with_string_key() {
    let array = Value::Array(vec![
        Value::String("A".to_string()),
        Value::String("B".to_string()),
    ]);
    assert_eq!(array.get("A"), None);
}

#[test]
fn test_get_on_number() {
    let number = Value::Number(Number { n: 42 });
    assert_eq!(number.get(0), None);
    assert_eq!(number.get("key"), None);
}

