// Answer 0

#[test]
fn test_get_existing_key() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct TestMap {
        data: indexmap::IndexMap<String, i32>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                data: indexmap::IndexMap::new(),
            }
        }

        fn insert(&mut self, key: String, value: i32) {
            self.data.insert(key, value);
        }

        fn get<Q>(&self, key: &Q) -> Option<&i32>
        where
            Q: ?Sized + Hash + indexmap::Equivalent<String>,
        {
            if let Some(i) = self.data.get_index_of(key) {
                let entry = &self.data.as_entries()[i];
                Some(&entry.value)
            } else {
                None
            }
        }
    }

    let mut map = TestMap::new();
    map.insert("test".to_string(), 42);
    
    assert_eq!(map.get("test"), Some(&42));
}

#[test]
fn test_get_non_existing_key() {
    struct TestMap {
        data: indexmap::IndexMap<String, i32>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                data: indexmap::IndexMap::new(),
            }
        }

        fn insert(&mut self, key: String, value: i32) {
            self.data.insert(key, value);
        }

        fn get<Q>(&self, key: &Q) -> Option<&i32>
        where
            Q: ?Sized + Hash + indexmap::Equivalent<String>,
        {
            if let Some(i) = self.data.get_index_of(key) {
                let entry = &self.data.as_entries()[i];
                Some(&entry.value)
            } else {
                None
            }
        }
    }

    let map = TestMap::new();
    
    assert_eq!(map.get("non_existing"), None);
}

#[test]
fn test_get_with_different_key_types() {
    struct TestMap {
        data: indexmap::IndexMap<String, i32>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                data: indexmap::IndexMap::new(),
            }
        }

        fn insert(&mut self, key: String, value: i32) {
            self.data.insert(key, value);
        }

        fn get<Q>(&self, key: &Q) -> Option<&i32>
        where
            Q: ?Sized + Hash + indexmap::Equivalent<String>,
        {
            if let Some(i) = self.data.get_index_of(key) {
                let entry = &self.data.as_entries()[i];
                Some(&entry.value)
            } else {
                None
            }
        }
    }

    let mut map = TestMap::new();
    map.insert("key".to_string(), 10);

    assert_eq!(map.get(&String::from("key")), Some(&10));
    assert_eq!(map.get(&"key"[..]), Some(&10));
}

