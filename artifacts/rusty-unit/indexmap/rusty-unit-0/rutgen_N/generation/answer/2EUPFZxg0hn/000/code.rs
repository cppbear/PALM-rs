// Answer 0

#[test]
fn test_capacity_with_non_empty_entries_and_indices() {
    struct TestStruct {
        indices: Vec<usize>,
        entries: Vec<usize>,
    }

    impl TestStruct {
        fn new(indices: Vec<usize>, entries: Vec<usize>) -> Self {
            TestStruct { indices, entries }
        }

        fn capacity(&self) -> usize {
            std::cmp::min(self.indices.capacity(), self.entries.capacity())
        }
    }

    let test_instance = TestStruct::new(vec![1, 2, 3], vec![4, 5, 6]);
    assert_eq!(test_instance.capacity(), 3);
}

#[test]
fn test_capacity_with_empty_entries_and_non_empty_indices() {
    struct TestStruct {
        indices: Vec<usize>,
        entries: Vec<usize>,
    }

    impl TestStruct {
        fn new(indices: Vec<usize>, entries: Vec<usize>) -> Self {
            TestStruct { indices, entries }
        }

        fn capacity(&self) -> usize {
            std::cmp::min(self.indices.capacity(), self.entries.capacity())
        }
    }

    let test_instance = TestStruct::new(vec![1, 2, 3], Vec::new());
    assert_eq!(test_instance.capacity(), 0);
}

#[test]
fn test_capacity_with_empty_entries_and_indices() {
    struct TestStruct {
        indices: Vec<usize>,
        entries: Vec<usize>,
    }

    impl TestStruct {
        fn new(indices: Vec<usize>, entries: Vec<usize>) -> Self {
            TestStruct { indices, entries }
        }

        fn capacity(&self) -> usize {
            std::cmp::min(self.indices.capacity(), self.entries.capacity())
        }
    }

    let test_instance = TestStruct::new(Vec::new(), Vec::new());
    assert_eq!(test_instance.capacity(), 0);
}

