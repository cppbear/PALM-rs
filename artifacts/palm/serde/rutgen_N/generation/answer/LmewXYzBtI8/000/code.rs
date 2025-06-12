// Answer 0

#[test]
fn test_size_hint_with_boundaries() {
    struct TestStruct {
        iter: std::ops::Range<usize>,
    }

    impl TestStruct {
        fn size_hint(&self) -> Option<usize> {
            size_hint::from_bounds(&self.iter)
        }
    }

    let test_instance = TestStruct {
        iter: 0..10,
    };

    assert_eq!(test_instance.size_hint(), Some(10));
}

#[test]
fn test_size_hint_empty_range() {
    struct TestStruct {
        iter: std::ops::Range<usize>,
    }

    impl TestStruct {
        fn size_hint(&self) -> Option<usize> {
            size_hint::from_bounds(&self.iter)
        }
    }

    let test_instance = TestStruct {
        iter: 5..5,
    };

    assert_eq!(test_instance.size_hint(), Some(0));
}

#[test]
fn test_size_hint_single_element() {
    struct TestStruct {
        iter: std::ops::Range<usize>,
    }

    impl TestStruct {
        fn size_hint(&self) -> Option<usize> {
            size_hint::from_bounds(&self.iter)
        }
    }

    let test_instance = TestStruct {
        iter: 3..4,
    };

    assert_eq!(test_instance.size_hint(), Some(1));
}

