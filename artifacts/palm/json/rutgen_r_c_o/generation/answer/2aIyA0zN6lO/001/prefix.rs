// Answer 0

#[test]
fn test_get_mut_existing_key_array() {
    let mut map = serde_json::Map::new();
    map.insert("key".to_owned(), serde_json::Value::Array(vec![serde_json::Value::Number(1.into()), serde_json::Value::Number(2.into())]));

    match map.entry("key") {
        Entry::Occupied(mut occupied) => {
            occupied.get_mut().as_array_mut().unwrap().push(serde_json::Value::Number(3.into()));
        }
        Entry::Vacant(_) => unimplemented!(),
    }
}

#[test]
fn test_get_mut_existing_key_object() {
    let mut map = serde_json::Map::new();
    let mut object = serde_json::Map::new();
    object.insert("inner_key".to_owned(), serde_json::Value::String("inner_value".to_string()));
    map.insert("key".to_owned(), serde_json::Value::Object(object));

    match map.entry("key") {
        Entry::Occupied(mut occupied) => {
            let value = occupied.get_mut().as_object_mut().unwrap();
            value.insert("new_inner_key".to_owned(), serde_json::Value::String("new_inner_value".to_string()));
        }
        Entry::Vacant(_) => unimplemented!(),
    }
}

#[test]
fn test_get_mut_multiple_entries() {
    let mut map = serde_json::Map::new();
    
    for i in 0..10 {
        map.insert(format!("key{}", i), serde_json::Value::Array(vec![]));
    }

    for i in 0..10 {
        match map.entry(format!("key{}", i)) {
            Entry::Occupied(mut occupied) => {
                occupied.get_mut().as_array_mut().unwrap().push(serde_json::Value::Number(i.into()));
            }
            Entry::Vacant(_) => unimplemented!(),
        }
    }
}

#[test]
#[should_panic]
fn test_get_mut_empty_key() {
    let mut map = serde_json::Map::new();

    match map.entry("") {
        Entry::Occupied(mut occupied) => {
            occupied.get_mut();
        }
        Entry::Vacant(_) => unimplemented!(),
    }
}

#[test]
fn test_get_mut_large_value() {
    let mut map = serde_json::Map::new();
    let large_array = serde_json::Value::Array((0..1000).map(|i| serde_json::Value::Number(i.into())).collect());
    map.insert("large_value".to_owned(), large_array);

    match map.entry("large_value") {
        Entry::Occupied(mut occupied) => {
            occupied.get_mut().as_array_mut().unwrap().push(serde_json::Value::Number(1000.into()));
        }
        Entry::Vacant(_) => unimplemented!(),
    }
}

#[test]
fn test_get_mut_nested_object() {
    let mut map = serde_json::Map::new();
    let mut inner_object = serde_json::Map::new();
    inner_object.insert("nested_key".to_owned(), serde_json::Value::String("nested_value".to_string()));
    map.insert("outer_key".to_owned(), serde_json::Value::Object(inner_object));

    match map.entry("outer_key") {
        Entry::Occupied(mut occupied) => {
            let value = occupied.get_mut().as_object_mut().unwrap();
            value.insert("new_nested_key".to_owned(), serde_json::Value::String("new_nested_value".to_string()));
        }
        Entry::Vacant(_) => unimplemented!(),
    }
}

