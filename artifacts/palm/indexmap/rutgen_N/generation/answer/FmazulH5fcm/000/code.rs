// Answer 0

#[test]
fn test_index_within_bounds() {
    struct TestSet {
        items: Vec<i32>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { items: vec![1, 2, 3] }
        }

        fn len(&self) -> usize {
            self.items.len()
        }

        fn get_index(&self, index: usize) -> Option<&i32> {
            self.items.get(index)
        }

        fn index(&self, index: usize) -> &i32 {
            self.get_index(index).unwrap_or_else(|| {
                panic!(
                    "index out of bounds: the len is {len} but the index is {index}",
                    len = self.len(),
                    index = index
                );
            })
        }
    }

    let set = TestSet::new();
    assert_eq!(*set.index(0), 1);
    assert_eq!(*set.index(1), 2);
    assert_eq!(*set.index(2), 3);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_index_out_of_bounds() {
    struct TestSet {
        items: Vec<i32>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { items: vec![1, 2, 3] }
        }

        fn len(&self) -> usize {
            self.items.len()
        }

        fn get_index(&self, index: usize) -> Option<&i32> {
            self.items.get(index)
        }

        fn index(&self, index: usize) -> &i32 {
            self.get_index(index).unwrap_or_else(|| {
                panic!(
                    "index out of bounds: the len is {len} but the index is {index}",
                    len = self.len(),
                    index = index
                );
            })
        }
    }

    let set = TestSet::new();
    let _ = set.index(3); // This should panic
}

