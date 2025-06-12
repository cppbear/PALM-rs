// Answer 0

#[test]
fn test_index_or_insert_with_object() {
    use crate::Value;
    use crate::map::Map;

    let key = String::from("key");
    let mut value = Value::Object(Map::new());

    let result = key.index_or_insert(&mut value);

    if let Value::Object(ref mut map) = value {
        assert!(map.contains_key(&key));
        assert_eq!(result, &mut map[&key]);
    } else {
        panic!("Expected an Object variant.");
    }
}

#[test]
fn test_index_or_insert_with_array() {
    use crate::Value;

    let key = String::from("0");
    let mut value = Value::Array(vec![]);

    let result = key.index_or_insert(&mut value);

    if let Value::Array(ref mut array) = value {
        assert_eq!(array.len(), 1);
        assert_eq!(result, &mut array[0]);
    } else {
        panic!("Expected an Array variant.");
    }
}

#[test]
fn test_index_or_insert_with_null() {
    use crate::Value;

    let key = String::from("key");
    let mut value = Value::Null;

    let result = key.index_or_insert(&mut value);

    assert_eq!(value, Value::Object(Map::from([(key.clone(), Value::Null)])));
    if let Value::Object(ref mut map) = value {
        assert!(map.contains_key(&key));
        assert_eq!(result, &mut map[&key]);
    } else {
        panic!("Expected an Object variant.");
    }
}

