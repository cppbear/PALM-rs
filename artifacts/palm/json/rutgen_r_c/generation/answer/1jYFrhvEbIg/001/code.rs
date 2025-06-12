// Answer 0

#[test]
fn test_get_mut_object_key_exists() {
    let mut object = Value::Object(Map::new());
    object.as_object_mut().unwrap().insert("A".to_string(), Value::Number(Number { n: 1 }));
    
    let value = object.get_mut("A");
    assert!(value.is_some());
    assert_eq!(*value.unwrap(), Value::Number(Number { n: 1 }));
}

#[test]
fn test_get_mut_object_key_does_not_exist() {
    let mut object = Value::Object(Map::new());

    let value = object.get_mut("A");
    assert!(value.is_none());
}

#[test]
fn test_get_mut_array_index_within_bounds() {
    let mut array = Value::Array(vec![Value::Number(Number { n: 1 }), Value::Number(Number { n: 2 })]);
    
    let value = array.get_mut(1);
    assert!(value.is_some());
    assert_eq!(*value.unwrap(), Value::Number(Number { n: 2 }));
}

#[test]
fn test_get_mut_array_index_out_of_bounds() {
    let mut array = Value::Array(vec![Value::Number(Number { n: 1 }), Value::Number(Number { n: 2 })]);
    
    let value = array.get_mut(2);
    assert!(value.is_none());
}

#[test]
fn test_get_mut_number() {
    let mut number_value = Value::Number(Number { n: 1 });
    
    let value = number_value.get_mut(0);
    assert!(value.is_none());
}

#[test]
fn test_get_mut_read_only_on_value() {
    let mut value = Value::Null;

    let object_mut = value.get_mut("key");
    assert!(object_mut.is_none());
    let array_mut = value.get_mut(0);
    assert!(array_mut.is_none());
}

