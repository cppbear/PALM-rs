// Answer 0

#[test]
fn test_size_hint_none_case() {
    struct TestStruct {
        iter: std::iter::FromFn<fn() -> Option<usize>>,
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
        iter: std::iter::from_fn(|| None),
    };

    assert_eq!(test_instance.size_hint(), None);
}

#[test]
fn test_size_hint_boundary_case() {
    struct TestStruct {
        iter: std::iter::FromFn<fn() -> Option<(usize, Option<usize>)>>,
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
        iter: std::iter::from_fn(|| Some((5, Some(10)))),
    };

    assert_eq!(test_instance.size_hint(), None);
}

