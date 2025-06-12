// Answer 0

#[test]
fn test_size_hint_none() {
    struct TestIter {
        data: Vec<i32>,
    }

    impl Iterator for TestIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if !self.data.is_empty() {
                Some(self.data.remove(0))
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

    let test_iter = TestIter { data: vec![1, 2, 3] };
    let test_struct = TestStruct { iter: test_iter };
    assert_eq!(test_struct.size_hint(), None);
}

#[test]
fn test_size_hint_none_empty_iterator() {
    struct TestIter {
        data: Vec<i32>,
    }

    impl Iterator for TestIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if !self.data.is_empty() {
                Some(self.data.remove(0))
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

    let test_iter = TestIter { data: vec![] };
    let test_struct = TestStruct { iter: test_iter };
    assert_eq!(test_struct.size_hint(), None);
}

