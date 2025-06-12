// Answer 0

#[test]
fn test_size_hint_none_case() {
    struct TestIter {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let value = self.data[self.index];
                self.index += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    struct TestStruct {
        iter: TestIter,
    }

    impl TestStruct {
        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let test_data = TestStruct {
        iter: TestIter {
            data: vec![1, 2, 3],
            index: 0,
        },
    };

    assert_eq!(test_data.size_hint(), None);
}

#[test]
fn test_size_hint_none_empty_case() {
    struct TestIter {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let value = self.data[self.index];
                self.index += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    struct TestStruct {
        iter: TestIter,
    }

    impl TestStruct {
        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let test_data = TestStruct {
        iter: TestIter {
            data: vec![],
            index: 0,
        },
    };

    assert_eq!(test_data.size_hint(), None);
}

