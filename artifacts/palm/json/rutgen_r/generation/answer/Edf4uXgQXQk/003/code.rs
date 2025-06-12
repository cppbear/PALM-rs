// Answer 0

#[test]
fn test_size_hint_none_due_to_lower_not_equal_upper() {
    struct TestIterator {
        count: usize,
        limit: usize,
    }

    impl Iterator for TestIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.limit {
                self.count += 1;
                Some(self.count)
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

    let iterator = TestIterator { count: 0, limit: 5 };
    let test_struct = TestStruct { iter: iterator };
    assert_eq!(test_struct.size_hint(), None);
}

