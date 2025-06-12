// Answer 0

#[test]
fn test_shift_remove_index_valid() {
    struct TestMap {
        map: IndexMap<i32, String, RandomState>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                map: IndexMap::new(),
            }
        }

        fn insert(&mut self, key: i32, value: String) {
            self.map.insert(key, value);
        }

        fn length(&self) -> usize {
            self.map.len()
        }

        fn shift_remove_index(&mut self, index: usize) -> Option<(i32, String)> {
            self.map.shift_remove_index(index)
        }
    }

    let mut test_map = TestMap::new();
    test_map.insert(1, "one".to_string());
    test_map.insert(2, "two".to_string());
    test_map.insert(3, "three".to_string());

    let result = test_map.shift_remove_index(1);
    assert_eq!(result, Some((2, "two".to_string())));
    assert_eq!(test_map.length(), 2);
}

#[test]
fn test_shift_remove_index_boundary_conditions() {
    struct TestMap {
        map: IndexMap<i32, String, RandomState>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                map: IndexMap::new(),
            }
        }

        fn insert(&mut self, key: i32, value: String) {
            self.map.insert(key, value);
        }

        fn shift_remove_index(&mut self, index: usize) -> Option<(i32, String)> {
            self.map.shift_remove_index(index)
        }
    }

    let mut test_map = TestMap::new();
    test_map.insert(1, "one".to_string());
    test_map.insert(2, "two".to_string());

    let result_last = test_map.shift_remove_index(1);
    assert_eq!(result_last, Some((2, "two".to_string())));
    assert_eq!(test_map.length(), 1);

    let result_first = test_map.shift_remove_index(0);
    assert_eq!(result_first, Some((1, "one".to_string())));
    assert_eq!(test_map.length(), 0);
}

#[should_panic]
#[test]
fn test_shift_remove_index_out_of_bounds() {
    struct TestMap {
        map: IndexMap<i32, String, RandomState>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                map: IndexMap::new(),
            }
        }

        fn insert(&mut self, key: i32, value: String) {
            self.map.insert(key, value);
        }

        fn shift_remove_index(&mut self, index: usize) -> Option<(i32, String)> {
            self.map.shift_remove_index(index)
        }
    }

    let mut test_map = TestMap::new();
    test_map.insert(1, "one".to_string());

    // Testing out of bounds access
    test_map.shift_remove_index(1); // This should panic
}

