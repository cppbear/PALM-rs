// Answer 0

#[test]
fn test_as_slice_non_empty() {
    struct TestMap {
        iter: Vec<(i32, &str)>,
    }

    impl TestMap {
        fn as_slice(&self) -> &[(i32, &str)] {
            &self.iter
        }
    }

    let map = TestMap {
        iter: vec![(1, "a"), (2, "b"), (3, "c")],
    };

    let slice = map.as_slice();
    assert_eq!(slice.len(), 3);
    assert_eq!(slice[0], (1, "a"));
    assert_eq!(slice[1], (2, "b"));
    assert_eq!(slice[2], (3, "c"));
}

#[test]
fn test_as_slice_empty() {
    struct TestMap {
        iter: Vec<(i32, &str)>,
    }

    impl TestMap {
        fn as_slice(&self) -> &[(i32, &str)] {
            &self.iter
        }
    }

    let map = TestMap {
        iter: vec![],
    };

    let slice = map.as_slice();
    assert_eq!(slice.len(), 0);
}

