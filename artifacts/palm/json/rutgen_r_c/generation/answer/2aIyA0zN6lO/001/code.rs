// Answer 0

#[test]
fn test_get_mut_existing_entry() {
    use serde_json::json;
    use serde_json::Map;
    
    let mut map = Map::new();
    map.insert("key1".to_owned(), json!(42));
    
    let entry = map.entry("key1");
    match entry {
        Entry::Occupied(mut occupied) => {
            assert_eq!(occupied.get_mut(), &mut json!(42));
            *occupied.get_mut() = json!(100);
        }
        Entry::Vacant(_) => panic!("Expected occupied entry"),
    }
    
    assert_eq!(map["key1"], json!(100));
}

#[test]
#[should_panic(expected = "Expected occupied entry")]
fn test_get_mut_vacant_entry() {
    use serde_json::json;
    use serde_json::Map;
    
    let mut map = Map::new();
    
    let entry = map.entry("key2");
    match entry {
        Entry::Occupied(mut occupied) => {
            // This code should not execute as the entry is vacant
            let _value = occupied.get_mut();
        }
        Entry::Vacant(_) => panic!("Expected occupied entry"),
    }
}

#[test]
fn test_get_mut_modify_entry() {
    use serde_json::json;
    use serde_json::Map;
    
    let mut map = Map::new();
    map.insert("key3".to_owned(), json!(["a", "b", "c"]));
    
    let entry = map.entry("key3");
    match entry {
        Entry::Occupied(mut occupied) => {
            occupied.get_mut().as_array_mut().unwrap().push(json!("d"));
            assert_eq!(occupied.get().as_array().unwrap().len(), 4);
        }
        Entry::Vacant(_) => panic!("Expected occupied entry"),
    }
    
    assert_eq!(map["key3"].as_array().unwrap().len(), 4);
}

