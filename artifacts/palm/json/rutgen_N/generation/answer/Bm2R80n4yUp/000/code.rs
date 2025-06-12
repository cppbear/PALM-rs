// Answer 0

#[test]
fn test_get_existing_key() {
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::borrow::Borrow;

    struct Value(String);
    struct MyMap {
        map: HashMap<String, Value>,
    }

    impl MyMap {
        pub fn get<Q>(&self, key: &Q) -> Option<&Value>
        where
            String: Borrow<Q>,
            Q: ?Sized + Ord + Eq + Hash,
        {
            self.map.get(key)
        }
    }

    let mut map = HashMap::new();
    map.insert("key1".to_string(), Value("value1".to_string()));
    let my_map = MyMap { map };

    let result = my_map.get("key1");
    assert!(result.is_some());
    assert_eq!(result.unwrap().0, "value1");
}

#[test]
fn test_get_non_existing_key() {
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::borrow::Borrow;

    struct Value(String);
    struct MyMap {
        map: HashMap<String, Value>,
    }

    impl MyMap {
        pub fn get<Q>(&self, key: &Q) -> Option<&Value>
        where
            String: Borrow<Q>,
            Q: ?Sized + Ord + Eq + Hash,
        {
            self.map.get(key)
        }
    }

    let mut map = HashMap::new();
    map.insert("key1".to_string(), Value("value1".to_string()));
    let my_map = MyMap { map };

    let result = my_map.get("non_existing_key");
    assert!(result.is_none());
}

#[test]
fn test_get_with_borrowed_key() {
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::borrow::Borrow;

    struct Value(String);
    struct MyMap {
        map: HashMap<String, Value>,
    }

    impl MyMap {
        pub fn get<Q>(&self, key: &Q) -> Option<&Value>
        where
            String: Borrow<Q>,
            Q: ?Sized + Ord + Eq + Hash,
        {
            self.map.get(key)
        }
    }

    let mut map = HashMap::new();
    map.insert("key1".to_string(), Value("value1".to_string()));
    let my_map = MyMap { map };

    let key: String = "key1".to_string();
    let result = my_map.get(&key);
    assert!(result.is_some());
    assert_eq!(result.unwrap().0, "value1");
}

