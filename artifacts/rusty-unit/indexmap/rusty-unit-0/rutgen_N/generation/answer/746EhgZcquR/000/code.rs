// Answer 0

#[test]
fn test_swap_remove() {
    struct TestMap {
        data: indexmap::IndexMap<i32, String>,
    }
    
    impl TestMap {
        fn new() -> Self {
            TestMap {
                data: indexmap::IndexMap::new(),
            }
        }

        fn insert(&mut self, key: i32, value: String) {
            self.data.insert(key, value);
        }

        fn swap_remove_entry(self, key: i32) -> Option<(i32, String)> {
            let entry = self.data.shift_remove(&key)?;
            Some((entry.0, entry.1))
        }

        fn swap_remove(self, key: i32) -> Option<String> {
            let entry = self.swap_remove_entry(key)?;
            Some(entry.1)
        }
    }

    let mut map = TestMap::new();
    map.insert(1, "value1".to_string());
    map.insert(2, "value2".to_string());
    map.insert(3, "value3".to_string());

    let value = map.swap_remove(2).unwrap();
    assert_eq!(value, "value2");

    assert!(map.swap_remove(2).is_none());
} 

#[test]
fn test_swap_remove_last_element() {
    struct TestMap {
        data: indexmap::IndexMap<i32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                data: indexmap::IndexMap::new(),
            }
        }

        fn insert(&mut self, key: i32, value: String) {
            self.data.insert(key, value);
        }

        fn swap_remove_entry(self, key: i32) -> Option<(i32, String)> {
            let entry = self.data.shift_remove(&key)?;
            Some((entry.0, entry.1))
        }

        fn swap_remove(self, key: i32) -> Option<String> {
            let entry = self.swap_remove_entry(key)?;
            Some(entry.1)
        }
    }

    let mut map = TestMap::new();
    map.insert(1, "value1".to_string());
    map.insert(2, "value2".to_string());

    let value = map.swap_remove(1).unwrap();
    assert_eq!(value, "value1");

    let last_value = map.swap_remove(2).unwrap();
    assert_eq!(last_value, "value2");
} 

#[test]
#[should_panic]
fn test_swap_remove_non_existent() {
    struct TestMap {
        data: indexmap::IndexMap<i32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                data: indexmap::IndexMap::new(),
            }
        }

        fn insert(&mut self, key: i32, value: String) {
            self.data.insert(key, value);
        }

        fn swap_remove_entry(self, key: i32) -> Option<(i32, String)> {
            let entry = self.data.shift_remove(&key)?;
            Some((entry.0, entry.1))
        }

        fn swap_remove(self, key: i32) -> Option<String> {
            let entry = self.swap_remove_entry(key)?;
            Some(entry.1)
        }
    }

    let mut map = TestMap::new();
    map.insert(1, "value1".to_string());
    map.swap_remove(1).unwrap();
    map.swap_remove(1).unwrap(); // This should panic since key 1 no longer exists.
}

