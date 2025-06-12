// Answer 0

#[test]
fn test_index_empty_map() {
    struct TestMap {
        indices: Vec<usize>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { indices: Vec::new() }
        }
        
        fn index(&self) -> usize {
            self.indices.len()
        }
    }

    let map = TestMap::new();
    assert_eq!(map.index(), 0);
}

#[test]
fn test_index_non_empty_map() {
    struct TestMap {
        indices: Vec<usize>,
    }

    impl TestMap {
        fn new(indices: Vec<usize>) -> Self {
            TestMap { indices }
        }
        
        fn index(&self) -> usize {
            self.indices.len()
        }
    }

    let map = TestMap::new(vec![1, 2, 3]);
    assert_eq!(map.index(), 3);
}

