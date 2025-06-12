// Answer 0

#[test]
fn test_binary_search_by_key_existing_key() {
    struct TestMap {
        entries: Vec<(i32, String)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                entries: vec![(1, "one".to_string()), (2, "two".to_string()), (3, "three".to_string())],
            }
        }

        fn as_slice(&self) -> &[(i32, String)] {
            &self.entries
        }
    }

    let map = TestMap::new();
    let result = map.binary_search_by_key(&2, |&key, _| key);
    assert_eq!(result, Ok(1));
}

#[test]
fn test_binary_search_by_key_insertion_point() {
    struct TestMap {
        entries: Vec<(i32, String)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                entries: vec![(1, "one".to_string()), (3, "three".to_string()), (4, "four".to_string())],
            }
        }

        fn as_slice(&self) -> &[(i32, String)] {
            &self.entries
        }
    }

    let map = TestMap::new();
    let result = map.binary_search_by_key(&2, |&key, _| key);
    assert_eq!(result, Err(1));
}

#[test]
fn test_binary_search_by_key_empty_map() {
    struct TestMap {
        entries: Vec<(i32, String)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: vec![] }
        }

        fn as_slice(&self) -> &[(i32, String)] {
            &self.entries
        }
    }

    let map = TestMap::new();
    let result = map.binary_search_by_key(&1, |&key, _| key);
    assert_eq!(result, Err(0));
}

