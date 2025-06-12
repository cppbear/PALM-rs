// Answer 0

#[test]
fn test_size_hint_equal_bounds() {
    struct TestIter {
        data: Vec<i32>,
    }

    impl TestIter {
        fn new(data: Vec<i32>) -> Self {
            Self { data }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let lower = self.data.len();
            let upper = self.data.len();
            (lower, Some(upper))
        }
    }

    struct TestStruct {
        iter: TestIter,
    }

    impl TestStruct {
        fn new(iter: TestIter) -> Self {
            Self { iter }
        }

        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let test_data = vec![1, 2, 3, 4, 5];
    let test_iter = TestIter::new(test_data);
    let test_struct = TestStruct::new(test_iter);

    assert_eq!(test_struct.size_hint(), Some(5));
}

#[test]
fn test_size_hint_zero_elements() {
    struct TestIter {
        data: Vec<i32>,
    }

    impl TestIter {
        fn new(data: Vec<i32>) -> Self {
            Self { data }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let lower = self.data.len();
            let upper = self.data.len();
            (lower, Some(upper))
        }
    }

    struct TestStruct {
        iter: TestIter,
    }

    impl TestStruct {
        fn new(iter: TestIter) -> Self {
            Self { iter }
        }

        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let test_data: Vec<i32> = vec![];
    let test_iter = TestIter::new(test_data);
    let test_struct = TestStruct::new(test_iter);

    assert_eq!(test_struct.size_hint(), Some(0));
}

