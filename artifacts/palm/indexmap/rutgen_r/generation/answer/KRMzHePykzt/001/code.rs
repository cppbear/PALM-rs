// Answer 0

#[test]
fn test_get_none_when_key_absent() {
    use std::collections::HashMap;
    use std::hash::Hash;

    struct MyMap<K, V> {
        map: HashMap<K, V>,
    }

    impl<K: Eq + Hash, V> MyMap<K, V> {
        pub fn new() -> Self {
            MyMap {
                map: HashMap::new(),
            }
        }

        pub fn get<Q>(&self, key: &Q) -> Option<&V>
        where
            Q: ?Sized + Hash + std::cmp::Eq,
        {
            if let Some(i) = self.map.get(key) {
                Some(i)
            } else {
                None
            }
        }
    }

    let my_map: MyMap<String, i32> = MyMap::new();
    let result = my_map.get(&"nonexistent_key".to_string());
    assert_eq!(result, None);
}

#[test]
fn test_get_none_when_key_absent_with_integer() {
    struct MyMap<K, V> {
        map: HashMap<K, V>,
    }

    impl<K: Eq + Hash, V> MyMap<K, V> {
        pub fn new() -> Self {
            MyMap {
                map: HashMap::new(),
            }
        }

        pub fn get<Q>(&self, key: &Q) -> Option<&V>
        where
            Q: ?Sized + Hash + std::cmp::Eq,
        {
            if let Some(i) = self.map.get(key) {
                Some(i)
            } else {
                None
            }
        }
    }

    let my_map: MyMap<i32, String> = MyMap::new();
    let result = my_map.get(&42);
    assert_eq!(result, None);
}

#[test]
fn test_get_none_when_key_absent_with_custom_struct() {
    #[derive(Debug, Hash, PartialEq, Eq)]
    struct Key {
        id: i32,
    }

    struct MyMap<K, V> {
        map: HashMap<K, V>,
    }

    impl<K: Eq + Hash, V> MyMap<K, V> {
        pub fn new() -> Self {
            MyMap {
                map: HashMap::new(),
            }
        }

        pub fn get<Q>(&self, key: &Q) -> Option<&V>
        where
            Q: ?Sized + Hash + std::cmp::Eq,
        {
            if let Some(i) = self.map.get(key) {
                Some(i)
            } else {
                None
            }
        }
    }

    let my_map: MyMap<Key, String> = MyMap::new();
    let result = my_map.get(&Key { id: 1 });
    assert_eq!(result, None);
}

