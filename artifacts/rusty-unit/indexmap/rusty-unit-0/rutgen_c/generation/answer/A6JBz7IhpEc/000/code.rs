// Answer 0

#[test]
fn test_binary_search_keys_found() {
    struct TestMap {
        entries: Vec<(i32, &'static str)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: vec![(1, "a"), (2, "b"), (3, "c")] }
        }

        fn binary_search_keys(&self, x: &i32) -> Result<usize, usize> {
            let mut start = 0;
            let mut end = self.entries.len();

            while start < end {
                let mid = (start + end) / 2;
                if &self.entries[mid].0 < x {
                    start = mid + 1;
                } else if &self.entries[mid].0 > x {
                    end = mid;
                } else {
                    return Ok(mid);
                }
            }
            Err(start)
        }
    }

    let map = TestMap::new();
    assert_eq!(map.binary_search_keys(&2), Ok(1));
}

#[test]
fn test_binary_search_keys_not_found() {
    struct TestMap {
        entries: Vec<(i32, &'static str)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: vec![(1, "a"), (2, "b"), (3, "c")] }
        }

        fn binary_search_keys(&self, x: &i32) -> Result<usize, usize> {
            let mut start = 0;
            let mut end = self.entries.len();

            while start < end {
                let mid = (start + end) / 2;
                if &self.entries[mid].0 < x {
                    start = mid + 1;
                } else if &self.entries[mid].0 > x {
                    end = mid;
                } else {
                    return Ok(mid);
                }
            }
            Err(start)
        }
    }

    let map = TestMap::new();
    assert_eq!(map.binary_search_keys(&4), Err(3));
}

