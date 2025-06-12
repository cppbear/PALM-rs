// Answer 0

#[test]
fn test_contains_key_with_existing_key() {
    use std::collections::HashMap;
    use std::hash::Hash;

    struct MyMap {
        map: HashMap<String, i32>,
    }

    impl MyMap {
        pub fn new() -> Self {
            MyMap {
                map: HashMap::new(),
            }
        }

        pub fn insert(&mut self, key: String, value: i32) {
            self.map.insert(key, value);
        }

        pub fn contains_key<Q>(&self, key: &Q) -> bool
        where
            String: Borrow<Q>,
            Q: ?Sized + Ord + Eq + Hash,
        {
            self.map.contains_key(key)
        }
    }

    let mut my_map = MyMap::new();
    my_map.insert("key1".to_string(), 10);
    assert!(my_map.contains_key("key1"));
}

#[test]
fn test_contains_key_with_non_existing_key() {
    use std::collections::HashMap;
    use std::hash::Hash;

    struct MyMap {
        map: HashMap<String, i32>,
    }

    impl MyMap {
        pub fn new() -> Self {
            MyMap {
                map: HashMap::new(),
            }
        }

        pub fn insert(&mut self, key: String, value: i32) {
            self.map.insert(key, value);
        }

        pub fn contains_key<Q>(&self, key: &Q) -> bool
        where
            String: Borrow<Q>,
            Q: ?Sized + Ord + Eq + Hash,
        {
            self.map.contains_key(key)
        }
    }

    let mut my_map = MyMap::new();
    my_map.insert("key1".to_string(), 10);
    assert!(!my_map.contains_key("key2"));
}

#[test]
fn test_contains_key_with_borrowed_str() {
    use std::collections::HashMap;
    use std::hash::Hash;

    struct MyMap {
        map: HashMap<String, i32>,
    }

    impl MyMap {
        pub fn new() -> Self {
            MyMap {
                map: HashMap::new(),
            }
        }

        pub fn insert(&mut self, key: String, value: i32) {
            self.map.insert(key, value);
        }

        pub fn contains_key<Q>(&self, key: &Q) -> bool
        where
            String: Borrow<Q>,
            Q: ?Sized + Ord + Eq + Hash,
        {
            self.map.contains_key(key)
        }
    }

    let mut my_map = MyMap::new();
    my_map.insert("key1".to_string(), 10);
    let borrowed_key: &str = "key1";
    assert!(my_map.contains_key(borrowed_key));
}

#[test]
fn test_contains_key_with_empty_map() {
    use std::collections::HashMap;
    use std::hash::Hash;

    struct MyMap {
        map: HashMap<String, i32>,
    }

    impl MyMap {
        pub fn new() -> Self {
            MyMap {
                map: HashMap::new(),
            }
        }

        pub fn contains_key<Q>(&self, key: &Q) -> bool
        where
            String: Borrow<Q>,
            Q: ?Sized + Ord + Eq + Hash,
        {
            self.map.contains_key(key)
        }
    }

    let my_map = MyMap::new();
    assert!(!my_map.contains_key("any_key"));
}

