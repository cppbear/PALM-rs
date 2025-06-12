// Answer 0

#[test]
fn test_index_mut_existing_key() {
    let mut map = Map {
        map: MapImpl::new(),
    };
    map.map.insert("key".to_string(), Value::Number(Number::from(42)));

    let value = map.index_mut(&"key".to_string());
    if let Value::Number(n) = value {
        assert_eq!(*n, 42);
    } else {
        panic!("Expected a Number variant");
    }
}

#[test]
#[should_panic(expected = "no entry found for key")]
fn test_index_mut_non_existing_key() {
    let mut map = Map {
        map: MapImpl::new(),
    };
    // No key is inserted, so this should panic
    let _value = map.index_mut(&"non_existing_key".to_string());
}

#[test]
fn test_index_mut_multiple_inserts() {
    let mut map = Map {
        map: MapImpl::new(),
    };
    map.map.insert("key1".to_string(), Value::Number(Number::from(10)));
    map.map.insert("key2".to_string(), Value::Number(Number::from(20)));

    {
        let value = map.index_mut(&"key1".to_string());
        if let Value::Number(n) = value {
            *n = Number::from(100); // Update value
        }
    }

    {
        let value = map.index_mut(&"key2".to_string());
        if let Value::Number(n) = value {
            assert_eq!(*n, 20);
        } else {
            panic!("Expected a Number variant");
        }
    }

    {
        let value = map.index_mut(&"key1".to_string());
        if let Value::Number(n) = value {
            assert_eq!(*n, 100);
        } else {
            panic!("Expected a Number variant");
        }
    }
}

