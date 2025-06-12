// Answer 0

#[test]
fn test_remove_entry_single_element() {
    let mut map = serde_json::Map::new();
    map.insert("single_key".to_owned(), serde_json::Value::Number(serde_json::Number::from(1)));
    
    match map.entry("single_key") {
        serde_json::map::Entry::Occupied(occupied) => {
            let (key, value) = occupied.remove_entry();
        }
        serde_json::map::Entry::Vacant(_) => unimplemented!(),
    }
}

#[test]
fn test_remove_entry_multiple_elements() {
    let mut map = serde_json::Map::new();
    for i in 1..=10 {
        map.insert(format!("key_{}", i), serde_json::Value::String(format!("value_{}", i)));
    }
    
    match map.entry("key_5") {
        serde_json::map::Entry::Occupied(occupied) => {
            let (key, value) = occupied.remove_entry();
        }
        serde_json::map::Entry::Vacant(_) => unimplemented!(),
    }
}

#[test]
fn test_remove_entry_with_boolean_value() {
    let mut map = serde_json::Map::new();
    map.insert("bool_key".to_owned(), serde_json::Value::Bool(true));
    
    match map.entry("bool_key") {
        serde_json::map::Entry::Occupied(occupied) => {
            let (key, value) = occupied.remove_entry();
        }
        serde_json::map::Entry::Vacant(_) => unimplemented!(),
    }
}

#[test]
fn test_remove_entry_with_array_value() {
    let mut map = serde_json::Map::new();
    map.insert("array_key".to_owned(), serde_json::Value::Array(vec![serde_json::Value::Number(serde_json::Number::from(1)), serde_json::Value::Number(serde_json::Number::from(2))]));
    
    match map.entry("array_key") {
        serde_json::map::Entry::Occupied(occupied) => {
            let (key, value) = occupied.remove_entry();
        }
        serde_json::map::Entry::Vacant(_) => unimplemented!(),
    }
}

#[test]
fn test_remove_entry_with_object_value() {
    let mut map = serde_json::Map::new();
    let mut object_value = serde_json::Map::new();
    object_value.insert("inner_key".to_owned(), serde_json::Value::String("inner_value".to_owned()));
    map.insert("object_key".to_owned(), serde_json::Value::Object(object_value));
    
    match map.entry("object_key") {
        serde_json::map::Entry::Occupied(occupied) => {
            let (key, value) = occupied.remove_entry();
        }
        serde_json::map::Entry::Vacant(_) => unimplemented!(),
    }
}

#[test]
fn test_remove_entry_with_null_value() {
    let mut map = serde_json::Map::new();
    map.insert("null_key".to_owned(), serde_json::Value::Null);
    
    match map.entry("null_key") {
        serde_json::map::Entry::Occupied(occupied) => {
            let (key, value) = occupied.remove_entry();
        }
        serde_json::map::Entry::Vacant(_) => unimplemented!(),
    }
}

#[test]
fn test_remove_entry_key_length_edge_case() {
    let mut map = serde_json::Map::new();
    let long_key = "a".repeat(100);
    map.insert(long_key.clone(), serde_json::Value::Number(serde_json::Number::from(100)));
    
    match map.entry(&long_key) {
        serde_json::map::Entry::Occupied(occupied) => {
            let (key, value) = occupied.remove_entry();
        }
        serde_json::map::Entry::Vacant(_) => unimplemented!(),
    }
}

#[test]
fn test_remove_entry_order_preserved() {
    #[cfg(feature = "preserve_order")]
    {
        let mut map = indexmap::IndexMap::new();
        for i in 1..=10 {
            map.insert(format!("key_{}", i), serde_json::Value::String(format!("value_{}", i)));
        }
        
        match map.entry("key_5") {
            serde_json::map::Entry::Occupied(occupied) => {
                let (key, value) = occupied.remove_entry();
            }
            serde_json::map::Entry::Vacant(_) => unimplemented!(),
        }
    }
}

#[test]
fn test_remove_entry_key_uniqueness() {
    let mut map = serde_json::Map::new();
    map.insert("unique_key".to_owned(), serde_json::Value::Bool(false));
    
    match map.entry("unique_key") {
        serde_json::map::Entry::Occupied(occupied) => {
            let (key, value) = occupied.remove_entry();
        }
        serde_json::map::Entry::Vacant(_) => unimplemented!(),
    }
}

#[test]
#[should_panic]
fn test_remove_entry_non_existent_key() {
    let mut map = serde_json::Map::new();
    map.insert("existing_key".to_owned(), serde_json::Value::Number(serde_json::Number::from(42)));
    
    match map.entry("non_existent_key") {
        serde_json::map::Entry::Occupied(occupied) => {
            let (key, value) = occupied.remove_entry();
        }
        serde_json::map::Entry::Vacant(_) => unimplemented!(),
    }
}

