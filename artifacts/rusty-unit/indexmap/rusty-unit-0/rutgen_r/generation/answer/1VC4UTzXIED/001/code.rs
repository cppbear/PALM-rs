// Answer 0

#[test]
fn test_binary_search_by_key_found() {
    struct TestMap {
        data: Vec<(i32, &str)>,
    }
    
    impl TestMap {
        fn binary_search_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Result<usize, usize>
        where
            F: FnMut(&'a i32, &'a &str) -> B,
            B: Ord,
        {
            self.data.binary_search_by_key(b, f)
        }

        fn new(data: Vec<(i32, &str)>) -> Self {
            TestMap { data }
        }

        fn as_slice(&self) -> &[(i32, &str)] {
            &self.data
        }
    }

    let map = TestMap::new(vec![(1, "one"), (2, "two"), (3, "three")]);
    let result = map.binary_search_by_key(&2, |k, _| *k);
    assert_eq!(result, Ok(1));
}

#[test]
fn test_binary_search_by_key_not_found() {
    struct TestMap {
        data: Vec<(i32, &str)>,
    }

    impl TestMap {
        fn binary_search_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Result<usize, usize>
        where
            F: FnMut(&'a i32, &'a &str) -> B,
            B: Ord,
        {
            self.data.binary_search_by_key(b, f)
        }

        fn new(data: Vec<(i32, &str)>) -> Self {
            TestMap { data }
        }

        fn as_slice(&self) -> &[(i32, &str)] {
            &self.data
        }
    }

    let map = TestMap::new(vec![(1, "one"), (3, "three")]);
    let result = map.binary_search_by_key(&2, |k, _| *k);
    assert_eq!(result, Err(1));
}

#[test]
fn test_binary_search_by_key_empty() {
    struct TestMap {
        data: Vec<(i32, &str)>,
    }

    impl TestMap {
        fn binary_search_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Result<usize, usize>
        where
            F: FnMut(&'a i32, &'a &str) -> B,
            B: Ord,
        {
            self.data.binary_search_by_key(b, f)
        }

        fn new(data: Vec<(i32, &str)>) -> Self {
            TestMap { data }
        }

        fn as_slice(&self) -> &[(i32, &str)] {
            &self.data
        }
    }

    let map = TestMap::new(vec![]);
    let result = map.binary_search_by_key(&1, |k, _| *k);
    assert_eq!(result, Err(0));
}

#[test]
fn test_binary_search_by_key_first_element() {
    struct TestMap {
        data: Vec<(i32, &str)>,
    }

    impl TestMap {
        fn binary_search_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Result<usize, usize>
        where
            F: FnMut(&'a i32, &'a &str) -> B,
            B: Ord,
        {
            self.data.binary_search_by_key(b, f)
        }

        fn new(data: Vec<(i32, &str)>) -> Self {
            TestMap { data }
        }

        fn as_slice(&self) -> &[(i32, &str)] {
            &self.data
        }
    }

    let map = TestMap::new(vec![(1, "one"), (2, "two"), (3, "three")]);
    let result = map.binary_search_by_key(&1, |k, _| *k);
    assert_eq!(result, Ok(0));
}

#[test]
fn test_binary_search_by_key_last_element() {
    struct TestMap {
        data: Vec<(i32, &str)>,
    }

    impl TestMap {
        fn binary_search_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Result<usize, usize>
        where
            F: FnMut(&'a i32, &'a &str) -> B,
            B: Ord,
        {
            self.data.binary_search_by_key(b, f)
        }

        fn new(data: Vec<(i32, &str)>) -> Self {
            TestMap { data }
        }

        fn as_slice(&self) -> &[(i32, &str)] {
            &self.data
        }
    }

    let map = TestMap::new(vec![(1, "one"), (2, "two"), (3, "three")]);
    let result = map.binary_search_by_key(&3, |k, _| *k);
    assert_eq!(result, Ok(2));
}

