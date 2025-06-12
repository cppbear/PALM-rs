// Answer 0

#[test]
fn test_binary_search_keys_present() {
    struct TestMap {
        keys: Vec<i32>,
    }

    impl TestMap {
        fn as_slice(&self) -> &[i32] {
            &self.keys
        }

        fn binary_search_keys(&self, x: &i32) -> Result<usize, usize> {
            self.as_slice().binary_search(x)
        }
    }

    let map = TestMap { keys: vec![1, 2, 3, 4, 5] };
    let result = map.binary_search_keys(&3);
    assert_eq!(result, Ok(2));
}

#[test]
fn test_binary_search_keys_not_present() {
    struct TestMap {
        keys: Vec<i32>,
    }

    impl TestMap {
        fn as_slice(&self) -> &[i32] {
            &self.keys
        }

        fn binary_search_keys(&self, x: &i32) -> Result<usize, usize> {
            self.as_slice().binary_search(x)
        }
    }

    let map = TestMap { keys: vec![1, 2, 3, 4, 5] };
    let result = map.binary_search_keys(&6);
    assert_eq!(result, Err(5));
}

#[test]
fn test_binary_search_keys_edge_case_empty() {
    struct TestMap {
        keys: Vec<i32>,
    }

    impl TestMap {
        fn as_slice(&self) -> &[i32] {
            &self.keys
        }

        fn binary_search_keys(&self, x: &i32) -> Result<usize, usize> {
            self.as_slice().binary_search(x)
        }
    }

    let map = TestMap { keys: Vec::new() };
    let result = map.binary_search_keys(&1);
    assert_eq!(result, Err(0));
}

#[test]
fn test_binary_search_keys_edge_case_first_element() {
    struct TestMap {
        keys: Vec<i32>,
    }

    impl TestMap {
        fn as_slice(&self) -> &[i32] {
            &self.keys
        }

        fn binary_search_keys(&self, x: &i32) -> Result<usize, usize> {
            self.as_slice().binary_search(x)
        }
    }

    let map = TestMap { keys: vec![2, 4, 6, 8] };
    let result = map.binary_search_keys(&2);
    assert_eq!(result, Ok(0));
}

#[test]
fn test_binary_search_keys_edge_case_last_element() {
    struct TestMap {
        keys: Vec<i32>,
    }

    impl TestMap {
        fn as_slice(&self) -> &[i32] {
            &self.keys
        }

        fn binary_search_keys(&self, x: &i32) -> Result<usize, usize> {
            self.as_slice().binary_search(x)
        }
    }

    let map = TestMap { keys: vec![2, 4, 6, 8] };
    let result = map.binary_search_keys(&8);
    assert_eq!(result, Ok(3));
}

