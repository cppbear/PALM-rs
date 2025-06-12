// Answer 0

#[test]
fn test_size_hint_equal_lower_upper() {
    struct TestIterator {
        count: usize,
        limit: usize,
    }
    
    impl Iterator for TestIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.limit {
                self.count += 1;
                Some(self.count - 1)
            } else {
                None
            }
        }
    }

    struct TestStruct {
        iter: TestIterator,
    }

    impl TestStruct {
        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let iter = TestIterator { count: 0, limit: 5 };
    let test_struct = TestStruct { iter };
    
    assert_eq!(test_struct.size_hint(), Some(5));
}

#[test]
fn test_size_hint_zero_elements() {
    struct TestIterator {
        count: usize,
    }
    
    impl Iterator for TestIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    struct TestStruct {
        iter: TestIterator,
    }

    impl TestStruct {
        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let iter = TestIterator { count: 0 };
    let test_struct = TestStruct { iter };
    
    assert_eq!(test_struct.size_hint(), None);
}

