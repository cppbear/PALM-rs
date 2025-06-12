// Answer 0

#[test]
fn test_size_hint_none_lower_not_equal_upper() {
    struct TestIter {
        data: Vec<i32>,
    }

    impl TestIter {
        fn size_hint(&self) -> (usize, Option<usize>) {
            (3, Some(5)) // lower != upper
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

    let test_struct = TestStruct {
        iter: TestIter { data: vec![1, 2, 3] },
    };

    assert_eq!(test_struct.size_hint(), None);
}

