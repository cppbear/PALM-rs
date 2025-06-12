// Answer 0

#[test]
fn test_size_hint_none() {
    struct TestIter {
        data: Vec<i32>,
        index: usize,
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
        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }
    
    let test_iter = TestIter { data: vec![1, 2, 3], index: 0 }; // Here the size hint will be (3, Some(3))
    let test_struct = TestStruct { iter: test_iter };

    assert_eq!(test_struct.size_hint(), None); // we expect None because the size hint matches the constraint.
}

#[test]
fn test_size_hint_lower_greater_than_upper() {
    struct TestIter {
        data: Vec<i32>,
        index: usize,
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
        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }
    
    let test_iter = TestIter { data: vec![1, 2], index: 0 }; // Here the size hint will be (2, Some(2))
    let test_struct = TestStruct { iter: test_iter };

    assert_eq!(test_struct.size_hint(), None); // we expect None because it doesn't fulfill the size hint criteria.
}

#[test]
fn test_size_hint_upper_none() {
    struct TestIter {
        data: Vec<i32>,
        index: usize,
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
        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }
    
    let test_iter = TestIter { data: vec![], index: 0 }; // Empty iterator, size hint is (0, Some(0))
    let test_struct = TestStruct { iter: test_iter };

    assert_eq!(test_struct.size_hint(), None); // we expect None because the size hint does not fulfill the equality criterion.
}

