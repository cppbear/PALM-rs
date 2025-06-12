// Answer 0

#[test]
fn test_binary_search_by_key_found() {
    struct TestSet {
        data: Vec<i32>,
    }

    impl TestSet {
        fn as_slice(&self) -> &[i32] {
            &self.data
        }

        pub fn binary_search_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Result<usize, usize>
        where
            F: FnMut(&'a i32) -> B,
            B: Ord,
        {
            self.as_slice().binary_search_by_key(b, f)
        }
    }

    let set = TestSet { data: vec![1, 2, 3, 4, 5] };
    let result = set.binary_search_by_key(&3, |&&x| x);
    assert_eq!(result, Ok(2));
}

#[test]
fn test_binary_search_by_key_not_found() {
    struct TestSet {
        data: Vec<i32>,
    }

    impl TestSet {
        fn as_slice(&self) -> &[i32] {
            &self.data
        }

        pub fn binary_search_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Result<usize, usize>
        where
            F: FnMut(&'a i32) -> B,
            B: Ord,
        {
            self.as_slice().binary_search_by_key(b, f)
        }
    }

    let set = TestSet { data: vec![1, 2, 4, 5] };
    let result = set.binary_search_by_key(&3, |&&x| x);
    assert_eq!(result, Err(2));
}

#[test]
fn test_binary_search_by_key_empty_set() {
    struct TestSet {
        data: Vec<i32>,
    }

    impl TestSet {
        fn as_slice(&self) -> &[i32] {
            &self.data
        }

        pub fn binary_search_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Result<usize, usize>
        where
            F: FnMut(&'a i32) -> B,
            B: Ord,
        {
            self.as_slice().binary_search_by_key(b, f)
        }
    }

    let set = TestSet { data: vec![] };
    let result = set.binary_search_by_key(&1, |&&x| x);
    assert_eq!(result, Err(0));
}

#[test]
fn test_binary_search_by_key_boundary() {
    struct TestSet {
        data: Vec<i32>,
    }

    impl TestSet {
        fn as_slice(&self) -> &[i32] {
            &self.data
        }

        pub fn binary_search_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Result<usize, usize>
        where
            F: FnMut(&'a i32) -> B,
            B: Ord,
        {
            self.as_slice().binary_search_by_key(b, f)
        }
    }

    let set = TestSet { data: vec![1, 2, 3, 4, 5] };
    let result = set.binary_search_by_key(&1, |&&x| x);
    assert_eq!(result, Ok(0));

    let result = set.binary_search_by_key(&5, |&&x| x);
    assert_eq!(result, Ok(4));
}

