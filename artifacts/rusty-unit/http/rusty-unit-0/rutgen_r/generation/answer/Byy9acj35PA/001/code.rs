// Answer 0

#[test]
fn test_clear_with_non_empty_extensions() {
    struct Extensions {
        map: Option<std::collections::HashMap<String, i32>>,
    }

    impl Extensions {
        pub fn new() -> Self {
            Extensions { map: Some(std::collections::HashMap::new()) }
        }

        pub fn insert(&mut self, key: String, value: i32) {
            if let Some(ref mut map) = self.map {
                map.insert(key, value);
            }
        }

        pub fn clear(&mut self) {
            if let Some(ref mut map) = self.map {
                map.clear();
            }
        }

        pub fn get<T: std::any::Any>(&self) -> Option<&T> {
            if let Some(ref map) = self.map {
                for value in map.values() {
                    if let Some(v) = value.downcast_ref::<T>() {
                        return Some(v);
                    }
                }
            }
            None
        }
    }

    let mut ext = Extensions::new();
    ext.insert("key1".to_string(), 5);
    ext.insert("key2".to_string(), 10);
    ext.clear();

    assert!(ext.get::<i32>().is_none());
}

#[test]
fn test_clear_with_empty_extensions() {
    struct Extensions {
        map: Option<std::collections::HashMap<String, i32>>,
    }

    impl Extensions {
        pub fn new() -> Self {
            Extensions { map: Some(std::collections::HashMap::new()) }
        }

        pub fn clear(&mut self) {
            if let Some(ref mut map) = self.map {
                map.clear();
            }
        }

        pub fn get<T: std::any::Any>(&self) -> Option<&T> {
            if let Some(ref map) = self.map {
                for value in map.values() {
                    if let Some(v) = value.downcast_ref::<T>() {
                        return Some(v);
                    }
                }
            }
            None
        }
    }

    let mut ext = Extensions::new();
    ext.clear();

    assert!(ext.get::<i32>().is_none());
}

