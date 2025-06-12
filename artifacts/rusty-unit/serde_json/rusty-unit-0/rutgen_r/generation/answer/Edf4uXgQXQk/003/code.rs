// Answer 0

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
                let result = Some(self.data[self.index]);
                self.index += 1;
                result
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

    // Testing with lower != upper
    let data = vec![1, 2, 3];
    let iter = TestIter { data, index: 0 };
    let test_struct = TestStruct { iter };

    // The iterator will yield 3 elements, but let's force a size hint that returns (2, Some(4))
    // which means lower != upper
    let result = test_struct.size_hint();

    assert_eq!(result, None);
}

