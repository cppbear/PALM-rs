// Answer 0

#[test]
fn test_get_key_value_existing_key() {
    #[derive(Hash, Eq, PartialEq)]
    struct Key {
        id: i32,
    }

    let mut map: HashMap<Key, &str> = HashMap::new();
    let key = Key { id: 1 };
    map.insert(key.clone(), "value");

    // Test retrieving the value using the original key
    assert_eq!(map.get_key_value(&key), Some((&key, &"value")));
}

#[test]
fn test_get_key_value_existing_key_borrowed() {
    #[derive(Hash, Eq, PartialEq)]
    struct Key {
        id: i32,
    }

    let mut map: HashMap<Key, &str> = HashMap::new();
    let key = Key { id: 2 };
    map.insert(key.clone(), "value");

    // Test retrieving using a borrowed form (reference)
    assert_eq!(map.get_key_value(&Key { id: 2 }), Some((&key, &"value")));
}

#[test]
fn test_get_key_value_non_existing_key() {
    #[derive(Hash, Eq, PartialEq)]
    struct Key {
        id: i32,
    }

    let mut map: HashMap<Key, &str> = HashMap::new();
    let key1 = Key { id: 3 };
    let key2 = Key { id: 4 };
    map.insert(key1.clone(), "value");

    // Test retrieving a non-existing key
    assert_eq!(map.get_key_value(&key2), None);
}

