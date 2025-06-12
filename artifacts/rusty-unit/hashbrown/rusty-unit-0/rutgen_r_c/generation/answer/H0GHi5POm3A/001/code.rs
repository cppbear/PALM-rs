// Answer 0

#[test]
fn test_get_existing_key() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct HashMapWrapper {
        map: HashMap<i32, &str, DefaultHasher>,
    }

    impl HashMapWrapper {
        fn new() -> Self {
            HashMapWrapper {
                map: HashMap::default(),
            }
        }

        fn insert(&mut self, k: i32, v: &str) {
            self.map.insert(k, v);
        }

        fn get(&self, k: &i32) -> Option<&str> {
            self.map.get(k)
        }
    }

    let mut map_wrapper = HashMapWrapper::new();
    map_wrapper.insert(1, "a");
    assert_eq!(map_wrapper.get(&1), Some("a"));
}

#[test]
fn test_get_non_existing_key() {
    use std::collections::hash_map::DefaultHasher;

    struct HashMapWrapper {
        map: HashMap<i32, &str, DefaultHasher>,
    }

    impl HashMapWrapper {
        fn new() -> Self {
            HashMapWrapper {
                map: HashMap::default(),
            }
        }

        fn insert(&mut self, k: i32, v: &str) {
            self.map.insert(k, v);
        }

        fn get(&self, k: &i32) -> Option<&str> {
            self.map.get(k)
        }
    }

    let mut map_wrapper = HashMapWrapper::new();
    map_wrapper.insert(1, "a");
    assert_eq!(map_wrapper.get(&2), None);
}

