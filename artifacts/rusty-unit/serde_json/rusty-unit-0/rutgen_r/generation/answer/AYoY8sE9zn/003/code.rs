// Answer 0

#[test]
fn test_size_hint_lower_not_equal_upper() {
    struct TestIter {
        values: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.values.len() {
                let value = self.values[self.index];
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

    let test_iter = TestIter { values: vec![1, 2, 3], index: 0 }; // Size hint will yield (3, Some(3))
    let test_struct = TestStruct { iter: test_iter };
    assert_eq!(test_struct.size_hint(), None); // Since lower != upper, we expect None
}

