// Answer 0

#[test]
fn test_index_valid() {
    struct TestIndexSet {
        elements: Vec<i32>,
    }

    impl TestIndexSet {
        fn new(elements: Vec<i32>) -> Self {
            Self { elements }
        }

        fn len(&self) -> usize {
            self.elements.len()
        }

        fn get_index(&self, index: usize) -> Option<&i32> {
            self.elements.get(index)
        }

        fn index(&self, index: usize) -> &i32 {
            self.get_index(index).unwrap_or_else(|| {
                panic!(
                    "index out of bounds: the len is {len} but the index is {index}",
                    len = self.len()
                );
            })
        }
    }

    let index_set = TestIndexSet::new(vec![10, 20, 30]);

    assert_eq!(*index_set.index(0), 10);
    assert_eq!(*index_set.index(1), 20);
    assert_eq!(*index_set.index(2), 30);
}

#[should_panic(expected = "index out of bounds")]
#[test]
fn test_index_out_of_bounds_lower() {
    struct TestIndexSet {
        elements: Vec<i32>,
    }

    impl TestIndexSet {
        fn new(elements: Vec<i32>) -> Self {
            Self { elements }
        }

        fn len(&self) -> usize {
            self.elements.len()
        }

        fn get_index(&self, index: usize) -> Option<&i32> {
            self.elements.get(index)
        }

        fn index(&self, index: usize) -> &i32 {
            self.get_index(index).unwrap_or_else(|| {
                panic!(
                    "index out of bounds: the len is {len} but the index is {index}",
                    len = self.len()
                );
            })
        }
    }

    let index_set = TestIndexSet::new(vec![10, 20, 30]);
    index_set.index(3);
}

#[should_panic(expected = "index out of bounds")]
#[test]
fn test_index_out_of_bounds_upper() {
    struct TestIndexSet {
        elements: Vec<i32>,
    }

    impl TestIndexSet {
        fn new(elements: Vec<i32>) -> Self {
            Self { elements }
        }

        fn len(&self) -> usize {
            self.elements.len()
        }

        fn get_index(&self, index: usize) -> Option<&i32> {
            self.elements.get(index)
        }

        fn index(&self, index: usize) -> &i32 {
            self.get_index(index).unwrap_or_else(|| {
                panic!(
                    "index out of bounds: the len is {len} but the index is {index}",
                    len = self.len()
                );
            })
        }
    }

    let index_set = TestIndexSet::new(vec![10, 20, 30]);
    index_set.index(usize::MAX);
}

