// Answer 0

#[test]
fn test_binary_search_keys_found() {
    struct TestMap {
        keys: Vec<i32>,
    }

    impl TestMap {
        fn binary_search_keys(&self, x: &i32) -> Result<usize, usize> {
            let mid = self.keys.binary_search(x);
            match mid {
                Ok(idx) => Ok(idx),
                Err(idx) => Err(idx),
            }
        }
    }

    let map = TestMap { keys: vec![1, 2, 3, 4, 5] };
    assert_eq!(map.binary_search_keys(&3), Ok(2));
}

#[test]
fn test_binary_search_keys_not_found() {
    struct TestMap {
        keys: Vec<i32>,
    }

    impl TestMap {
        fn binary_search_keys(&self, x: &i32) -> Result<usize, usize> {
            let mid = self.keys.binary_search(x);
            match mid {
                Ok(idx) => Ok(idx),
                Err(idx) => Err(idx),
            }
        }
    }

    let map = TestMap { keys: vec![1, 2, 3, 4, 5] };
    assert_eq!(map.binary_search_keys(&6), Err(5));
}

#[test]
fn test_binary_search_keys_empty() {
    struct TestMap {
        keys: Vec<i32>,
    }

    impl TestMap {
        fn binary_search_keys(&self, x: &i32) -> Result<usize, usize> {
            let mid = self.keys.binary_search(x);
            match mid {
                Ok(idx) => Ok(idx),
                Err(idx) => Err(idx),
            }
        }
    }

    let map = TestMap { keys: vec![] };
    assert_eq!(map.binary_search_keys(&1), Err(0));
}

