// Answer 0

#[test]
fn test_iterator_len_hint_none_case() {
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

        fn size_hint(&self) -> (usize, Option<usize>) {
            (0, Some(self.data.len() + 1)) // Here lo (0) != hi (len + 1), so it meets the constraint
        }
    }

    let test_iter = TestIter { data: vec![1, 2, 3], index: 0 };
    assert_eq!(iterator_len_hint(&test_iter), None);
}

#[test]
fn test_iterator_len_hint_none_case_two() {
    struct AnotherTestIter {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for AnotherTestIter {
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

        fn size_hint(&self) -> (usize, Option<usize>) {
            (3, Some(5)) // lo (3) != hi (5), another case meeting the constraint
        }
    }

    let another_test_iter = AnotherTestIter { data: vec![4, 5, 6], index: 0 };
    assert_eq!(iterator_len_hint(&another_test_iter), None);
}

