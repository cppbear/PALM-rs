// Answer 0

#[test]
fn test_insert_new_key() {
    struct TestMap {
        map: std::collections::HashMap<i32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { map: std::collections::HashMap::new() }
        }

        fn insert(&mut self, key: i32, value: String) -> Option<String> {
            self.map.insert(key, value)
        }
    }

    let mut map = TestMap::new();
    assert_eq!(map.insert(1, "value1".to_string()), None);
    assert_eq!(map.insert(2, "value2".to_string()), None);
}

#[test]
fn test_insert_existing_key() {
    struct TestMap {
        map: std::collections::HashMap<i32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { map: std::collections::HashMap::new() }
        }

        fn insert(&mut self, key: i32, value: String) -> Option<String> {
            self.map.insert(key, value)
        }
    }

    let mut map = TestMap::new();
    map.insert(1, "value1".to_string());
    assert_eq!(map.insert(1, "new_value1".to_string()), Some("value1".to_string()));
}

#[test]
fn test_insert_multiple_same_keys() {
    struct TestMap {
        map: std::collections::HashMap<i32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { map: std::collections::HashMap::new() }
        }

        fn insert(&mut self, key: i32, value: String) -> Option<String> {
            self.map.insert(key, value)
        }
    }

    let mut map = TestMap::new();
    assert_eq!(map.insert(1, "value1".to_string()), None);
    assert_eq!(map.insert(1, "new_value1".to_string()), Some("value1".to_string()));
    assert_eq!(map.insert(1, "another_value1".to_string()), Some("new_value1".to_string()));
}

