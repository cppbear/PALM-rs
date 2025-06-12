// Answer 0

#[test]
fn test_contains_key_existing_key() {
    use std::collections::HashMap;

    struct MyMap {
        map: HashMap<String, i32>,
    }

    let mut my_map = MyMap {
        map: HashMap::new(),
    };
    my_map.map.insert("key1".to_string(), 1);

    assert!(my_map.contains_key("key1"));
}

#[test]
fn test_contains_key_non_existing_key() {
    use std::collections::HashMap;

    struct MyMap {
        map: HashMap<String, i32>,
    }

    let mut my_map = MyMap {
        map: HashMap::new(),
    };
    my_map.map.insert("key1".to_string(), 1);

    assert!(!my_map.contains_key("key2"));
}

#[test]
fn test_contains_key_empty_map() {
    use std::collections::HashMap;

    struct MyMap {
        map: HashMap<String, i32>,
    }

    let my_map = MyMap {
        map: HashMap::new(),
    };

    assert!(!my_map.contains_key("any_key"));
}

#[test]
fn test_contains_key_with_different_reference() {
    use std::collections::HashMap;

    struct MyMap {
        map: HashMap<String, i32>,
    }

    let mut my_map = MyMap {
        map: HashMap::new(),
    };
    my_map.map.insert("key1".to_string(), 1);

    let key_ref: &str = "key1";
    assert!(my_map.contains_key(key_ref));
}

