// Answer 0

#[test]
fn test_insert_sorted_new_key() {
    struct TestMap {
        map: IndexMap<i32, String, RandomState>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                map: IndexMap::new(),
            }
        }

        fn insert_sorted(&mut self, key: i32, value: String) -> (usize, Option<String>) {
            self.map.insert_sorted(key, value)
        }
    }

    let mut test_map = TestMap::new();

    // Insert a key-value pair, expecting it to be inserted at index 0
    let (index1, return_value1) = test_map.insert_sorted(10, "Ten".to_string());
    assert_eq!(index1, 0);
    assert_eq!(return_value1, None);

    // Now, insert a key-value pair that should be placed before the first one
    let (index2, return_value2) = test_map.insert_sorted(5, "Five".to_string());
    assert_eq!(index2, 0);
    assert_eq!(return_value2, None);

    // Insert a new key that should be placed after the first element
    let (index3, return_value3) = test_map.insert_sorted(15, "Fifteen".to_string());
    assert_eq!(index3, 2);
    assert_eq!(return_value3, None);

    // Attempting to insert an existing key with a new value
    let (index4, return_value4) = test_map.insert_sorted(10, "New Ten".to_string());
    assert_eq!(index4, 1); // The index of the existing key
    assert_eq!(return_value4, Some("Ten".to_string())); // The previous value returned
}

#[test]
fn test_insert_sorted_empty_map() {
    struct TestMap {
        map: IndexMap<i32, String, RandomState>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                map: IndexMap::new(),
            }
        }

        fn insert_sorted(&mut self, key: i32, value: String) -> (usize, Option<String>) {
            self.map.insert_sorted(key, value)
        }
    }

    let mut test_map = TestMap::new();

    // Insert into an empty map; should allow insertion at index 0
    let (index1, return_value1) = test_map.insert_sorted(20, "Twenty".to_string());
    assert_eq!(index1, 0);
    assert_eq!(return_value1, None);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_insert_sorted_panic_index_out_of_bounds() {
    struct TestMap {
        map: IndexMap<i32, String, RandomState>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                map: IndexMap::new(),
            }
        }

        fn insert_sorted(&mut self, key: i32, value: String) -> (usize, Option<String>) {
            self.map.insert_sorted(key, value)
        }
    }

    let mut test_map = TestMap::new();
    // It should panic when trying to insert at a non-existent index
    let (index, return_value) = test_map.insert_sorted(30, "Thirty".to_string());
    assert_eq!(index, 0);
    assert_eq!(return_value, None);
    test_map.map.insert_sorted(31, "Thirty One".to_string()); // valid, now insert out of sorted order
    test_map.map.insert_sorted(32, "Thirty Two".to_string()); 
    // This will trigger panic in the next insert
    test_map.insert_sorted(31, "Thirty Updated".to_string());
}

