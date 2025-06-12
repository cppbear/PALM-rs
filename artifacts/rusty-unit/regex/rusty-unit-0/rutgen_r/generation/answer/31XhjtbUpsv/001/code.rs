// Answer 0

#[test]
fn test_iter_basic() {
    struct TestLocs {
        data: Vec<usize>, // Placeholder for capture locations
    }

    impl TestLocs {
        fn new(data: Vec<usize>) -> Self {
            TestLocs { data }
        }
        
        pub fn iter(&self) -> SubCapturesPosIter {
            SubCapturesPosIter { idx: 0, locs: self }
        }
    }

    struct SubCapturesPosIter<'a> {
        idx: usize,
        locs: &'a TestLocs,
    }

    // Create an instance of TestLocs
    let locs = TestLocs::new(vec![0, 1, 2, 3]);

    // Call the iter method
    let iter = locs.iter();

    // Check that the returned iterator has idx initialized to 0 and locs holding the correct reference
    assert_eq!(iter.idx, 0);
    assert_eq!(iter.locs, &locs);
}

#[test]
fn test_iter_empty() {
    struct TestLocs {
        data: Vec<usize>, // Placeholder for capture locations
    }

    impl TestLocs {
        fn new(data: Vec<usize>) -> Self {
            TestLocs { data }
        }
        
        pub fn iter(&self) -> SubCapturesPosIter {
            SubCapturesPosIter { idx: 0, locs: self }
        }
    }

    struct SubCapturesPosIter<'a> {
        idx: usize,
        locs: &'a TestLocs,
    }

    // Create an empty instance of TestLocs
    let locs = TestLocs::new(vec![]);

    // Call the iter method
    let iter = locs.iter();

    // Check that the returned iterator has idx initialized to 0 and locs holding the correct reference
    assert_eq!(iter.idx, 0);
    assert_eq!(iter.locs, &locs);
}

#[test]
fn test_iter_single_capture() {
    struct TestLocs {
        data: Vec<usize>, // Placeholder for capture locations
    }

    impl TestLocs {
        fn new(data: Vec<usize>) -> Self {
            TestLocs { data }
        }
        
        pub fn iter(&self) -> SubCapturesPosIter {
            SubCapturesPosIter { idx: 0, locs: self }
        }
    }

    struct SubCapturesPosIter<'a> {
        idx: usize,
        locs: &'a TestLocs,
    }

    // Create an instance of TestLocs with a single capture
    let locs = TestLocs::new(vec![5]);

    // Call the iter method
    let iter = locs.iter();

    // Check that the returned iterator has idx initialized to 0 and locs holding the correct reference
    assert_eq!(iter.idx, 0);
    assert_eq!(iter.locs, &locs);
}

