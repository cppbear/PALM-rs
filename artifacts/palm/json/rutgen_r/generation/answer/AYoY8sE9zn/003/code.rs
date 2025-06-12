// Answer 0

#[test]
fn test_size_hint_none_due_to_lower_not_equal_upper() {
    struct TestIter {
        lower: usize,
        upper: Option<usize>,
    }

    impl TestIter {
        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.lower, self.upper.clone())
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

    let test_instance = TestStruct {
        iter: TestIter { lower: 3, upper: Some(5) }, // lower != upper
    };

    assert_eq!(test_instance.size_hint(), None);
}

