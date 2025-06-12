// Answer 0

#[test]
fn test_capacity_non_empty() {
    struct TestIndexMap {
        indices: Vec<usize>,
        entries: Vec<usize>,
    }

    impl TestIndexMap {
        fn new(indices: Vec<usize>, entries: Vec<usize>) -> Self {
            Self { indices, entries }
        }

        fn capacity(&self) -> usize {
            std::cmp::min(self.indices.capacity(), self.entries.capacity())
        }
    }

    let map = TestIndexMap::new(vec![1, 2, 3], vec![4, 5, 6]);
    assert_eq!(map.capacity(), 3);
}

#[test]
fn test_capacity_empty() {
    struct TestIndexMap {
        indices: Vec<usize>,
        entries: Vec<usize>,
    }

    impl TestIndexMap {
        fn new(indices: Vec<usize>, entries: Vec<usize>) -> Self {
            Self { indices, entries }
        }

        fn capacity(&self) -> usize {
            std::cmp::min(self.indices.capacity(), self.entries.capacity())
        }
    }

    let map = TestIndexMap::new(vec![], vec![]);
    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_capacity_different_sizes() {
    struct TestIndexMap {
        indices: Vec<usize>,
        entries: Vec<usize>,
    }

    impl TestIndexMap {
        fn new(indices: Vec<usize>, entries: Vec<usize>) -> Self {
            Self { indices, entries }
        }

        fn capacity(&self) -> usize {
            std::cmp::min(self.indices.capacity(), self.entries.capacity())
        }
    }

    let map = TestIndexMap::new(vec![1, 2, 3], vec![4]);
    assert_eq!(map.capacity(), 1);
}

#[test]
fn test_capacity_with_panic_condition() {
    struct TestIndexMap {
        indices: Vec<usize>,
        entries: Vec<usize>,
    }

    impl TestIndexMap {
        fn new(indices: Vec<usize>, entries: Vec<usize>) -> Self {
            Self { indices, entries }
        }

        fn capacity(&self) -> usize {
            std::cmp::min(self.indices.capacity(), self.entries.capacity())
        }
    }

    let map = TestIndexMap::new(vec![1, 2, 3, 4, 5], vec![6]);
    assert!(map.capacity() <= 5);
}

