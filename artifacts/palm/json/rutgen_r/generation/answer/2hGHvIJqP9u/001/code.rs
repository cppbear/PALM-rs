// Answer 0

#[test]
fn test_is_empty_with_empty_map() {
    struct TestMap {
        map: std::collections::HashMap<String, String>,
    }

    impl TestMap {
        pub fn is_empty(&self) -> bool {
            self.map.is_empty()
        }
    }

    let test_map = TestMap {
        map: std::collections::HashMap::new(),
    };

    assert!(test_map.is_empty());
}

#[test]
fn test_is_empty_with_non_empty_map() {
    struct TestMap {
        map: std::collections::HashMap<String, String>,
    }

    impl TestMap {
        pub fn is_empty(&self) -> bool {
            self.map.is_empty()
        }
    }

    let mut test_map = TestMap {
        map: std::collections::HashMap::new(),
    };
    test_map.map.insert("key".to_string(), "value".to_string());

    assert!(!test_map.is_empty());
}

#[test]
fn test_is_empty_with_large_map() {
    struct TestMap {
        map: std::collections::HashMap<i32, i32>,
    }

    impl TestMap {
        pub fn is_empty(&self) -> bool {
            self.map.is_empty()
        }
    }

    let mut test_map = TestMap {
        map: std::collections::HashMap::new(),
    };
    
    for i in 0..1000 {
        test_map.map.insert(i, i * 2);
    }

    assert!(!test_map.is_empty());
}

#[test]
fn test_is_empty_with_single_element_map() {
    struct TestMap {
        map: std::collections::HashMap<String, String>,
    }

    impl TestMap {
        pub fn is_empty(&self) -> bool {
            self.map.is_empty()
        }
    }

    let mut test_map = TestMap {
        map: std::collections::HashMap::new(),
    };
    test_map.map.insert("single_key".to_string(), "single_value".to_string());
    
    assert!(!test_map.is_empty());
}

