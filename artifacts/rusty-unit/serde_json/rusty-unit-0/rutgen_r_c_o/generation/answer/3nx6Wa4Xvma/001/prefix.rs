// Answer 0

#[test]
fn test_into_mut_with_empty_value() {
    let mut map = serde_json::Map::new();
    map.insert("key".to_owned(), serde_json::Value::Null);

    match map.entry("key") {
        Entry::Occupied(mut occupied) => {
            occupied.into_mut();
        }
        Entry::Vacant(_) => unimplemented!(),
    }
}

#[test]
fn test_into_mut_with_bool_value() {
    let mut map = serde_json::Map::new();
    map.insert("key".to_owned(), serde_json::Value::Bool(false));

    match map.entry("key") {
        Entry::Occupied(mut occupied) => {
            occupied.into_mut();
        }
        Entry::Vacant(_) => unimplemented!(),
    }
}

#[test]
fn test_into_mut_with_number_value() {
    let mut map = serde_json::Map::new();
    map.insert("key".to_owned(), serde_json::Value::Number(serde_json::Number::from(0)));

    match map.entry("key") {
        Entry::Occupied(mut occupied) => {
            occupied.into_mut();
        }
        Entry::Vacant(_) => unimplemented!(),
    }
}

#[test]
fn test_into_mut_with_empty_array() {
    let mut map = serde_json::Map::new();
    map.insert("key".to_owned(), serde_json::Value::Array(vec![]));

    match map.entry("key") {
        Entry::Occupied(mut occupied) => {
            occupied.into_mut();
        }
        Entry::Vacant(_) => unimplemented!(),
    }
}

#[test]
fn test_into_mut_with_empty_object() {
    let mut map = serde_json::Map::new();
    map.insert("key".to_owned(), serde_json::Value::Object(serde_json::Map::new()));

    match map.entry("key") {
        Entry::Occupied(mut occupied) => {
            occupied.into_mut();
        }
        Entry::Vacant(_) => unimplemented!(),
    }
}

