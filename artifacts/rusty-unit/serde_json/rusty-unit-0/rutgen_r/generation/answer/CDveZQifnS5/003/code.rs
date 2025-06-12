// Answer 0

#[test]
fn test_size_hint_lower_not_equal_upper() {
    struct TestIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl TestIterator {
        fn new(data: Vec<i32>) -> Self {
            TestIterator { data, index: 0 }
        }
    }

    impl Iterator for TestIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let item = self.data[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    struct TestStruct {
        iter: TestIterator,
    }

    impl TestStruct {
        fn new(data: Vec<i32>) -> Self {
            TestStruct { iter: TestIterator::new(data) }
        }

        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let test_data = vec![1, 2, 3];
    let test_struct = TestStruct::new(test_data);
    assert_eq!(test_struct.size_hint(), None);
}

