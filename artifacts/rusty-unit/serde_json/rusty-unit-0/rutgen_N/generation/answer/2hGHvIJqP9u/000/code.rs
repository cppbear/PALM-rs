// Answer 0

#[test]
fn test_is_empty_with_no_elements() {
    struct TestMap {
        map: std::collections::HashMap<String, String>,
    }

    impl TestMap {
        pub fn new() -> Self {
            TestMap {
                map: std::collections::HashMap::new(),
            }
        }

        pub fn is_empty(&self) -> bool {
            self.map.is_empty()
        }
    }

    let test_map = TestMap::new();
    assert!(test_map.is_empty());
}

#[test]
fn test_is_empty_with_elements() {
    struct TestMap {
        map: std::collections::HashMap<String, String>,
    }
    
    impl TestMap {
        pub fn new() -> Self {
            TestMap {
                map: std::collections::HashMap::new(),
            }
        }

        pub fn insert(&mut self, key: String, value: String) {
            self.map.insert(key, value);
        }

        pub fn is_empty(&self) -> bool {
            self.map.is_empty()
        }
    }

    let mut test_map = TestMap::new();
    test_map.insert("key".to_string(), "value".to_string());
    assert!(!test_map.is_empty());
}

