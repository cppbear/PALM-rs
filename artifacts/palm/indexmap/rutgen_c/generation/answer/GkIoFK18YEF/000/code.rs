// Answer 0

#[test]
fn test_binary_search_by_found() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                entries: vec![(1, 10), (2, 20), (3, 30)],
            }
        }
        
        fn binary_search_by<F>(&self, mut f: F) -> Result<usize, usize>
        where
            F: FnMut(&i32, &i32) -> Ordering,
        {
            let slice = &self.entries;
            slice.binary_search_by(|(k, v)| f(k, v))
        }
    }

    let map = TestMap::new();
    let result = map.binary_search_by(|key, _| key.cmp(&2));
    assert_eq!(result, Ok(1));
}

#[test]
fn test_binary_search_by_not_found() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                entries: vec![(1, 10), (2, 20), (3, 30)],
            }
        }

        fn binary_search_by<F>(&self, mut f: F) -> Result<usize, usize>
        where
            F: FnMut(&i32, &i32) -> Ordering,
        {
            let slice = &self.entries;
            slice.binary_search_by(|(k, v)| f(k, v))
        }
    }

    let map = TestMap::new();
    let result = map.binary_search_by(|key, _| key.cmp(&4));
    assert_eq!(result, Err(3)); // 4 would go to index 3
}

#[test]
fn test_binary_search_by_edge_case() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                entries: vec![(1, 10), (2, 20), (3, 30)],
            }
        }

        fn binary_search_by<F>(&self, mut f: F) -> Result<usize, usize>
        where
            F: FnMut(&i32, &i32) -> Ordering,
        {
            let slice = &self.entries;
            slice.binary_search_by(|(k, v)| f(k, v))
        }
    }

    let map = TestMap::new();
    let result = map.binary_search_by(|key, _| key.cmp(&1));
    assert_eq!(result, Ok(0)); // The first item
}

