// Answer 0

#[test]
fn test_binary_search_by_key_found() {
    struct TestMap {
        data: Vec<(i32, &str)>,
    }

    impl TestMap {
        fn new(data: Vec<(i32, &str)>) -> Self {
            TestMap { data }
        }

        fn as_slice(&self) -> &[(i32, &str)] {
            &self.data
        }

        pub fn binary_search_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Result<usize, usize>
        where
            F: FnMut(&'a i32, &'a &str) -> B,
            B: Ord,
        {
            self.as_slice().binary_search_by_key(b, f)
        }
    }

    let map = TestMap::new(vec![(1, "a"), (2, "b"), (3, "c")]);
    let result = map.binary_search_by_key(&2, |&k, _| *k);
    assert_eq!(result, Ok(1));
}

#[test]
fn test_binary_search_by_key_not_found() {
    struct TestMap {
        data: Vec<(i32, &str)>,
    }

    impl TestMap {
        fn new(data: Vec<(i32, &str)>) -> Self {
            TestMap { data }
        }

        fn as_slice(&self) -> &[(i32, &str)] {
            &self.data
        }

        pub fn binary_search_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Result<usize, usize>
        where
            F: FnMut(&'a i32, &'a &str) -> B,
            B: Ord,
        {
            self.as_slice().binary_search_by_key(b, f)
        }
    }

    let map = TestMap::new(vec![(1, "a"), (2, "b"), (3, "c")]);
    let result = map.binary_search_by_key(&4, |&k, _| *k);
    assert_eq!(result, Err(3));
}

#[test]
fn test_binary_search_by_key_boundary_case() {
    struct TestMap {
        data: Vec<(i32, &str)>,
    }

    impl TestMap {
        fn new(data: Vec<(i32, &str)>) -> Self {
            TestMap { data }
        }

        fn as_slice(&self) -> &[(i32, &str)] {
            &self.data
        }

        pub fn binary_search_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Result<usize, usize>
        where
            F: FnMut(&'a i32, &'a &str) -> B,
            B: Ord,
        {
            self.as_slice().binary_search_by_key(b, f)
        }
    }

    let map = TestMap::new(vec![(1, "a"), (2, "b"), (3, "c")]);
    let result = map.binary_search_by_key(&1, |&k, _| *k);
    assert_eq!(result, Ok(0));
    
    let result = map.binary_search_by_key(&3, |&k, _| *k);
    assert_eq!(result, Ok(2));
}

