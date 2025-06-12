// Answer 0

#[test]
fn test_or_insert_with_vacant_entry() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use indexmap::IndexMap;

    struct TestMap {
        map: IndexMap<String, i32, DefaultHasher>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                map: IndexMap::new(),
            }
        }

        fn or_insert_with<F>(&mut self, key: String, call: F) -> (&mut String, &mut i32)
        where
            F: FnOnce() -> (String, i32),
        {
            if let Some(value) = self.map.get_mut(&key) {
                (&mut key, value)
            } else {
                let (new_key, new_value) = call();
                self.map.insert(new_key, new_value);
                (&mut key, self.map.get_mut(&key).unwrap())
            }
        }
    }

    let mut test_map = TestMap::new();
    let (key, value) = test_map.or_insert_with("test".to_string(), || {
        ("test".to_string(), 42)
    });

    assert_eq!(*value, 42);
    assert_eq!(key, &"test".to_string());
}

#[test]
fn test_or_insert_with_occupied_entry() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use indexmap::IndexMap;

    struct TestMap {
        map: IndexMap<String, i32, DefaultHasher>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                map: IndexMap::new(),
            }
        }

        fn or_insert_with<F>(&mut self, key: String, call: F) -> (&mut String, &mut i32)
        where
            F: FnOnce() -> (String, i32),
        {
            if let Some(value) = self.map.get_mut(&key) {
                (&mut key, value)
            } else {
                let (new_key, new_value) = call();
                self.map.insert(new_key, new_value);
                (&mut key, self.map.get_mut(&key).unwrap())
            }
        }
    }

    let mut test_map = TestMap::new();
    test_map.or_insert_with("test".to_string(), || {
        ("test".to_string(), 42)
    });
    let (key, value) = test_map.or_insert_with("test".to_string(), || {
        ("test".to_string(), 100)
    });

    assert_eq!(*value, 42); // Should return existing value
    assert_eq!(key, &"test".to_string());
}

