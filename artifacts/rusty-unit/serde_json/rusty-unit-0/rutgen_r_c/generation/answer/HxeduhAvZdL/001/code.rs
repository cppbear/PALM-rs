// Answer 0

#[test]
fn test_insert_update_existing_value() {
    let mut map = serde_json::Map::new();
    map.insert("key1".to_owned(), serde_json::Value::Number(serde_json::Number::from(10)));

    match map.entry("key1") {
        serde_json::map::Entry::Occupied(mut occupied) => {
            let old_value = occupied.insert(serde_json::Value::Number(serde_json::Number::from(20)));
            assert_eq!(old_value, serde_json::Value::Number(10));
            assert_eq!(occupied.get(), &serde_json::Value::Number(20));
        }
        _ => panic!("Expected entry to be occupied"),
    }
}

#[test]
fn test_insert_into_vacant_entry() {
    let mut map = serde_json::Map::new();
    
    match map.entry("key2") {
        serde_json::map::Entry::Vacant(vacant) => {
            let old_value = vacant.insert(serde_json::Value::Number(serde_json::Number::from(30)));
            assert_eq!(old_value, serde_json::Value::Number(30));
            assert_eq!(map.get("key2"), Some(&serde_json::Value::Number(30)));
        }
        _ => panic!("Expected entry to be vacant"),
    }
}

#[test]
fn test_insert_with_different_value_type() {
    let mut map = serde_json::Map::new();
    map.insert("key3".to_owned(), serde_json::Value::String("initial".to_owned()));

    match map.entry("key3") {
        serde_json::map::Entry::Occupied(mut occupied) => {
            let old_value = occupied.insert(serde_json::Value::Bool(true));
            assert_eq!(old_value, serde_json::Value::String("initial".to_owned()));
            assert_eq!(occupied.get(), &serde_json::Value::Bool(true));
        }
        _ => panic!("Expected entry to be occupied"),
    }
}

#[test]
#[should_panic]
fn test_insert_on_empty_entry_should_panic() {
    let mut map = serde_json::Map::new();
    if let serde_json::map::Entry::Occupied(_) = map.entry("key4") {
        panic!("Expected entry to be vacant");
    }
}

#[test]
fn test_insert_for_multiple_keys() {
    let mut map = serde_json::Map::new();
    map.insert("key5".to_owned(), serde_json::Value::Null);
    map.insert("key6".to_owned(), serde_json::Value::Array(vec![]));

    match map.entry("key5") {
        serde_json::map::Entry::Occupied(mut occupied) => {
            let old_value = occupied.insert(serde_json::Value::Bool(false));
            assert_eq!(old_value, serde_json::Value::Null);
            assert_eq!(occupied.get(), &serde_json::Value::Bool(false));
        }
        _ => panic!("Expected entry to be occupied"),
    }
    
    match map.entry("key6") {
        serde_json::map::Entry::Occupied(mut occupied) => {
            let old_value = occupied.insert(serde_json::Value::Array(vec![serde_json::Value::String("item".to_owned())]));
            assert_eq!(old_value, serde_json::Value::Array(vec![]));
            assert_eq!(occupied.get(), &serde_json::Value::Array(vec![serde_json::Value::String("item".to_owned())]));
        }
        _ => panic!("Expected entry to be occupied"),
    }
}

