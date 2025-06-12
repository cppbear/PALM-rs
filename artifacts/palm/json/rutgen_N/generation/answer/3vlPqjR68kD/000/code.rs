// Answer 0

#[test]
fn test_size_hint_equal_lower_upper() {
    struct TestIter {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let result = self.data[self.index];
                self.index += 1;
                Some(result)
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
            TestStruct {
                iter: TestIter { data, index: 0 },
            }
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
    assert_eq!(test_struct.size_hint(), Some(3));
}

#[test]
fn test_size_hint_lower_not_equal_upper() {
    struct TestIter {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let result = self.data[self.index];
                self.index += 1;
                Some(result)
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
            TestStruct {
                iter: TestIter { data, index: 0 },
            }
        }

        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let test_data = vec![1]; // Only one element, so lower and upper are not equal
    let test_struct = TestStruct::new(test_data);
    assert_eq!(test_struct.size_hint(), None);
}

#[test]
fn test_size_hint_empty() {
    struct TestIter {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let result = self.data[self.index];
                self.index += 1;
                Some(result)
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
            TestStruct {
                iter: TestIter { data, index: 0 },
            }
        }

        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let test_data: Vec<i32> = vec![];
    let test_struct = TestStruct::new(test_data);
    assert_eq!(test_struct.size_hint(), None);
}

