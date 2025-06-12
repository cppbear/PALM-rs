// Answer 0

#[test]
fn test_swap_remove_index_valid() {
    struct TestMap {
        index_map: IndexMap<i32, String, RandomState>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                index_map: IndexMap::new(),
            }
        }

        fn insert(&mut self, key: i32, value: String) {
            self.index_map.insert_full(1, key, value);
        }
    }

    let mut map = TestMap::new();
    map.insert(0, "zero".to_string());
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());

    assert_eq!(map.index_map.swap_remove_index(1), Some((1, "one".to_string())));
    assert_eq!(map.index_map.get_index(1), Some((&2, &"two".to_string())));
}

#[test]
fn test_swap_remove_index_out_of_bounds() {
    struct TestMap {
        index_map: IndexMap<i32, String, RandomState>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                index_map: IndexMap::new(),
            }
        }

        fn insert(&mut self, key: i32, value: String) {
            self.index_map.insert_full(1, key, value);
        }
    }

    let mut map = TestMap::new();
    map.insert(0, "zero".to_string());

    assert!(map.index_map.swap_remove_index(1).is_none());
}

#[test]
fn test_swap_remove_index_single_element() {
    struct TestMap {
        index_map: IndexMap<i32, String, RandomState>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                index_map: IndexMap::new(),
            }
        }

        fn insert(&mut self, key: i32, value: String) {
            self.index_map.insert_full(1, key, value);
        }
    }

    let mut map = TestMap::new();
    map.insert(0, "zero".to_string());

    assert_eq!(map.index_map.swap_remove_index(0), Some((0, "zero".to_string())));
    assert!(map.index_map.get_index(0).is_none());
}

