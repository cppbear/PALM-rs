// Answer 0

#[test]
fn test_binary_search_keys_present() {
    struct TestMap {
        data: Vec<i32>,
    }

    impl TestMap {
        fn new(data: Vec<i32>) -> Self {
            TestMap { data }
        }

        fn as_slice(&self) -> &[i32] {
            &self.data
        }

        fn binary_search_keys(&self, x: &i32) -> Result<usize, usize> {
            self.as_slice().binary_search(x).map(|idx| idx as usize)
                .or_else(|idx| Ok(idx as usize))
        }
    }

    let map = TestMap::new(vec![1, 3, 5, 7, 9]);
    
    assert_eq!(map.binary_search_keys(&5), Ok(2));
    assert_eq!(map.binary_search_keys(&1), Ok(0));
    assert_eq!(map.binary_search_keys(&9), Ok(4));
}

#[test]
fn test_binary_search_keys_absent() {
    struct TestMap {
        data: Vec<i32>,
    }

    impl TestMap {
        fn new(data: Vec<i32>) -> Self {
            TestMap { data }
        }

        fn as_slice(&self) -> &[i32] {
            &self.data
        }

        fn binary_search_keys(&self, x: &i32) -> Result<usize, usize> {
            self.as_slice().binary_search(x).map(|idx| idx as usize)
                .or_else(|idx| Ok(idx as usize))
        }
    }

    let map = TestMap::new(vec![1, 3, 5, 7, 9]);
    
    assert_eq!(map.binary_search_keys(&4), Err(2));
    assert_eq!(map.binary_search_keys(&0), Err(0));
    assert_eq!(map.binary_search_keys(&10), Err(5));
}

#[test]
#[should_panic]
fn test_binary_search_keys_empty() {
    struct TestMap {
        data: Vec<i32>,
    }

    impl TestMap {
        fn new(data: Vec<i32>) -> Self {
            TestMap { data }
        }

        fn as_slice(&self) -> &[i32] {
            &self.data
        }

        fn binary_search_keys(&self, x: &i32) -> Result<usize, usize> {
            self.as_slice().binary_search(x).map(|idx| idx as usize)
                .or_else(|idx| Ok(idx as usize))
        }
    }

    let map = TestMap::new(vec![]);
    let _ = map.binary_search_keys(&5); // This should panic due to empty slice
}

