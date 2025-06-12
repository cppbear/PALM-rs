// Answer 0

#[test]
fn test_into_values_empty_map() {
    struct TestMap {
        map: std::collections::HashMap<String, i32>,
    }

    impl TestMap {
        pub fn new() -> Self {
            TestMap {
                map: std::collections::HashMap::new(),
            }
        }

        pub fn into_values(self) -> std::collections::hash_map::IntoValues<String, i32> {
            self.map.into_values()
        }
    }

    let test_map = TestMap::new();
    let values: Vec<i32> = test_map.into_values().collect();
    assert_eq!(values, Vec::<i32>::new());
}

#[test]
fn test_into_values_single_element() {
    struct TestMap {
        map: std::collections::HashMap<String, i32>,
    }

    impl TestMap {
        pub fn new() -> Self {
            let mut map = std::collections::HashMap::new();
            map.insert("key1".to_string(), 42);
            TestMap { map }
        }

        pub fn into_values(self) -> std::collections::hash_map::IntoValues<String, i32> {
            self.map.into_values()
        }
    }

    let test_map = TestMap::new();
    let values: Vec<i32> = test_map.into_values().collect();
    assert_eq!(values, vec![42]);
}

#[test]
fn test_into_values_multiple_elements() {
    struct TestMap {
        map: std::collections::HashMap<String, i32>,
    }

    impl TestMap {
        pub fn new() -> Self {
            let mut map = std::collections::HashMap::new();
            map.insert("key1".to_string(), 42);
            map.insert("key2".to_string(), 24);
            TestMap { map }
        }

        pub fn into_values(self) -> std::collections::hash_map::IntoValues<String, i32> {
            self.map.into_values()
        }
    }

    let test_map = TestMap::new();
    let values: Vec<i32> = test_map.into_values().collect();
    assert_eq!(values, vec![42, 24]);
}

