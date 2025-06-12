// Answer 0

#[test]
fn test_insert_new_key() {
    struct TestMap {
        map: std::collections::HashMap<i32, String>,
    }

    impl TestMap {
        fn insert(&mut self, key: i32, value: String) -> Option<String> {
            self.map.insert(key, value)
        }

        fn new() -> Self {
            TestMap {
                map: std::collections::HashMap::new(),
            }
        }
    }

    let mut test_map = TestMap::new();
    let result = test_map.insert(1, String::from("value1"));
    assert_eq!(result, None);
    assert_eq!(test_map.map.get(&1), Some(&String::from("value1")));
}

#[test]
fn test_insert_existing_key() {
    struct TestMap {
        map: std::collections::HashMap<i32, String>,
    }

    impl TestMap {
        fn insert(&mut self, key: i32, value: String) -> Option<String> {
            self.map.insert(key, value)
        }

        fn new() -> Self {
            TestMap {
                map: std::collections::HashMap::new(),
            }
        }
    }

    let mut test_map = TestMap::new();
    test_map.insert(1, String::from("value1"));
    let result = test_map.insert(1, String::from("value2"));
    assert_eq!(result, Some(String::from("value1")));
    assert_eq!(test_map.map.get(&1), Some(&String::from("value2")));
}

#[test]
fn test_insert_multiple_keys() {
    struct TestMap {
        map: std::collections::HashMap<i32, String>,
    }

    impl TestMap {
        fn insert(&mut self, key: i32, value: String) -> Option<String> {
            self.map.insert(key, value)
        }

        fn new() -> Self {
            TestMap {
                map: std::collections::HashMap::new(),
            }
        }
    }

    let mut test_map = TestMap::new();
    test_map.insert(1, String::from("value1"));
    test_map.insert(2, String::from("value2"));
    let result = test_map.insert(1, String::from("value3"));
    
    assert_eq!(result, Some(String::from("value1")));
    assert_eq!(test_map.map.len(), 2);
    assert_eq!(test_map.map.get(&1), Some(&String::from("value3")));
    assert_eq!(test_map.map.get(&2), Some(&String::from("value2")));
}

