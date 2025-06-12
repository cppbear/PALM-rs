// Answer 0

#[test]
fn test_insert_sorted_update_existing_key() {
    struct TestMap {
        data: IndexMap<u32, String, RandomState>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                data: IndexMap::new(),
            }
        }

        fn insert_sorted(&mut self, key: u32, value: String) -> (usize, Option<String>) {
            self.data.insert_sorted(key, value)
        }
    }

    let mut map = TestMap::new();
    map.insert_sorted(1, "one".to_string());
    let (index, old_value) = map.insert_sorted(1, "uno".to_string());
    assert_eq!(index, 0);
    assert_eq!(old_value, Some("one".to_string()));
}

#[test]
fn test_insert_sorted_insert_new_key() {
    struct TestMap {
        data: IndexMap<u32, String, RandomState>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                data: IndexMap::new(),
            }
        }

        fn insert_sorted(&mut self, key: u32, value: String) -> (usize, Option<String>) {
            self.data.insert_sorted(key, value)
        }
    }

    let mut map = TestMap::new();
    map.insert_sorted(1, "one".to_string());
    let (index, old_value) = map.insert_sorted(2, "two".to_string());
    assert_eq!(index, 1);
    assert_eq!(old_value, None);
}

#[test]
fn test_insert_sorted_multiple_keys() {
    struct TestMap {
        data: IndexMap<u32, String, RandomState>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                data: IndexMap::new(),
            }
        }

        fn insert_sorted(&mut self, key: u32, value: String) -> (usize, Option<String>) {
            self.data.insert_sorted(key, value)
        }
    }

    let mut map = TestMap::new();
    map.insert_sorted(3, "three".to_string());
    map.insert_sorted(1, "one".to_string());
    map.insert_sorted(2, "two".to_string());
    
    let (index, old_value) = map.insert_sorted(2, "dos".to_string());
    assert_eq!(index, 1);
    assert_eq!(old_value, Some("two".to_string()));
}

#[test]
fn test_insert_sorted_boundary_conditions() {
    struct TestMap {
        data: IndexMap<u32, String, RandomState>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                data: IndexMap::new(),
            }
        }

        fn insert_sorted(&mut self, key: u32, value: String) -> (usize, Option<String>) {
            self.data.insert_sorted(key, value)
        }
    }

    let mut map = TestMap::new();
    let (index_one, _) = map.insert_sorted(1, "one".to_string());
    let (index_zero, _) = map.insert_sorted(0, "zero".to_string());
    let (index_two, _) = map.insert_sorted(2, "two".to_string());

    assert_eq!(index_zero, 0);
    assert_eq!(index_one, 1);
    assert_eq!(index_two, 2);
}

