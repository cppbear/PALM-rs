// Answer 0

#[test]
fn test_size_hint_equal_lower_upper() {
    struct TestIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl TestIterator {
        fn new(data: Vec<i32>) -> Self {
            Self { data, index: 0 }
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
            Self {
                iter: TestIterator::new(data),
            }
        }

        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let test_struct = TestStruct::new(vec![1, 2, 3]);
    assert_eq!(test_struct.size_hint(), None);
}

#[test]
fn test_size_hint_lower_equals_upper() {
    struct TestIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl TestIterator {
        fn new(data: Vec<i32>) -> Self {
            Self { data, index: 0 }
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
            Self {
                iter: TestIterator::new(data),
            }
        }

        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let test_struct = TestStruct::new(vec![1, 2, 3, 4]);
    assert_eq!(test_struct.iter.size_hint(), (4, Some(4)));
    assert_eq!(test_struct.size_hint(), Some(4));
}

#[test]
fn test_size_hint_none() {
    struct TestIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl TestIterator {
        fn new(data: Vec<i32>) -> Self {
            Self { data, index: 0 }
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
            Self {
                iter: TestIterator::new(data),
            }
        }

        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let test_struct = TestStruct::new(vec![]);
    assert_eq!(test_struct.size_hint(), None);
}

