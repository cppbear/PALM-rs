// Answer 0

#[test]
fn test_size_hint_with_different_lower_upper() {
    struct TestStruct {
        iter: Vec<i32>,
    }

    impl TestStruct {
        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let test_instance = TestStruct { iter: vec![1, 2, 3, 4] }; // lower = 4, upper = 4
    assert_eq!(test_instance.size_hint(), None);
}

#[test]
fn test_size_hint_with_empty_iterator() {
    struct TestStruct {
        iter: Vec<i32>,
    }

    impl TestStruct {
        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let test_instance = TestStruct { iter: Vec::new() }; // lower = 0, upper = 0
    assert_eq!(test_instance.size_hint(), None);
}

#[test]
fn test_size_hint_with_non_finite_range() {
    struct TestStruct {
        iter: Vec<i32>,
    }

    impl TestStruct {
        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let test_instance = TestStruct { iter: vec![1, 2, 3, 4, 5] }; // lower = 5, upper = 5
    assert_eq!(test_instance.size_hint(), None);
}

