// Answer 0

#[test]
fn test_size_hint_some() {
    struct TestIter {
        iter: Vec<i32>,
    }

    impl TestIter {
        fn size_hint(&self) -> Option<usize> {
            size_hint::from_bounds(&self.iter)
        }
    }

    let test_iter = TestIter { iter: vec![1, 2, 3, 4, 5] };
    assert_eq!(test_iter.size_hint(), Some(5));
}

#[test]
fn test_size_hint_none() {
    struct TestIter {
        iter: Vec<i32>,
    }

    impl TestIter {
        fn size_hint(&self) -> Option<usize> {
            size_hint::from_bounds(&self.iter)
        }
    }

    let test_iter = TestIter { iter: vec![] };
    assert_eq!(test_iter.size_hint(), Some(0));
}

#[test]
#[should_panic]
fn test_size_hint_panic() {
    struct TestIter {
        iter: Vec<i32>,
    }

    impl TestIter {
        fn size_hint(&self) -> Option<usize> {
            self.iter.get(0).unwrap(); // This will panic if the vector is empty
            size_hint::from_bounds(&self.iter)
        }
    }

    let test_iter = TestIter { iter: vec![] };
    let _ = test_iter.size_hint(); // This will panic
}

