// Answer 0

#[test]
fn test_iter_empty() {
    struct MockLocs;

    impl MockLocs {
        fn new() -> Self {
            MockLocs
        }
    }

    let locs = MockLocs::new();
    let iter = locs.iter();
    assert_eq!(iter.idx, 0);
}

#[test]
fn test_iter_with_capture_groups() {
    struct MockLocs {
        captures: Vec<(usize, usize)>,
    }

    impl MockLocs {
        fn new(captures: Vec<(usize, usize)>) -> Self {
            MockLocs { captures }
        }

        fn iter(&self) -> SubCapturesPosIter {
            SubCapturesPosIter { idx: 0, locs: self }
        }
    }

    let locs = MockLocs::new(vec![(0, 1), (2, 3)]);
    let mut iter = locs.iter();
    assert_eq!(iter.idx, 0);
    // Additional assertions can be added based on expected outputs of the iterator
}

#[test]
#[should_panic]
fn test_iter_out_of_bounds() {
    struct MockLocs {
        captures: Vec<(usize, usize)>,
    }

    impl MockLocs {
        fn new(captures: Vec<(usize, usize)>) -> Self {
            MockLocs { captures }
        }

        fn iter(&self) -> SubCapturesPosIter {
            SubCapturesPosIter { idx: self.captures.len(), locs: self } // Intentional out-of-bounds
        }
    }

    let locs = MockLocs::new(vec![(0, 1), (2, 3)]);
    let iter = locs.iter();
    assert!(iter.idx >= locs.captures.len()); // This should trigger the panic
}

