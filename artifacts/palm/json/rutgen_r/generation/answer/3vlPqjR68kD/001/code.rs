// Answer 0

#[test]
fn test_size_hint_none() {
    struct TestIter {
        data: Vec<i32>,
        index: usize,
    }

    impl TestIter {
        fn new(data: Vec<i32>) -> Self {
            TestIter { data, index: 0 }
        }
    }

    impl Iterator for TestIter {
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
        iter: TestIter,
    }

    impl TestStruct {
        fn new(data: Vec<i32>) -> Self {
            TestStruct { iter: TestIter::new(data) }
        }

        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let test_data = vec![1, 2, 3]; // Lower and Upper don't match
    let test_struct = TestStruct::new(test_data);

    assert_eq!(test_struct.size_hint(), None);
}

