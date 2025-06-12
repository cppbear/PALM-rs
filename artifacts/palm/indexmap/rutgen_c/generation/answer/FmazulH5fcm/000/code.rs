// Answer 0

#[test]
fn test_index_valid_index() {
    struct TestIndexSet {
        data: Vec<i32>,
    }

    impl TestIndexSet {
        fn new(data: Vec<i32>) -> Self {
            Self { data }
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn get_index(&self, index: usize) -> Option<&i32> {
            self.data.get(index)
        }
        
        fn index(&self, index: usize) -> &i32 {
            self.get_index(index).unwrap_or_else(|| {
                panic!("index out of bounds: the len is {} but the index is {}", self.len(), index)
            })
        }
    }

    let index_set = TestIndexSet::new(vec![10, 20, 30]);
    assert_eq!(*index_set.index(1), 20);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 3 but the index is 3")]
fn test_index_out_of_bounds() {
    struct TestIndexSet {
        data: Vec<i32>,
    }

    impl TestIndexSet {
        fn new(data: Vec<i32>) -> Self {
            Self { data }
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn get_index(&self, index: usize) -> Option<&i32> {
            self.data.get(index)
        }
        
        fn index(&self, index: usize) -> &i32 {
            self.get_index(index).unwrap_or_else(|| {
                panic!("index out of bounds: the len is {} but the index is {}", self.len(), index)
            })
        }
    }

    let index_set = TestIndexSet::new(vec![10, 20, 30]);
    index_set.index(3); // This should panic
}

#[test]
fn test_index_empty() {
    struct TestIndexSet {
        data: Vec<i32>,
    }

    impl TestIndexSet {
        fn new(data: Vec<i32>) -> Self {
            Self { data }
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn get_index(&self, index: usize) -> Option<&i32> {
            self.data.get(index)
        }
        
        fn index(&self, index: usize) -> &i32 {
            self.get_index(index).unwrap_or_else(|| {
                panic!("index out of bounds: the len is {} but the index is {}", self.len(), index)
            })
        }
    }

    let index_set = TestIndexSet::new(vec![]);
    assert_eq!(index_set.len(), 0);
    assert!(std::panic::catch_unwind(|| index_set.index(0)).is_err());
}

