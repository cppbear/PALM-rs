// Answer 0

#[test]
fn test_len_with_single_match() {
    struct TestRegex {
        locs: Vec<usize>,
    }

    impl TestRegex {
        pub fn len(&self) -> usize {
            self.locs.len()
        }
    }

    let regex = TestRegex { locs: vec![0] };
    assert_eq!(regex.len(), 1);
}

#[test]
fn test_len_with_multiple_matches() {
    struct TestRegex {
        locs: Vec<usize>,
    }

    impl TestRegex {
        pub fn len(&self) -> usize {
            self.locs.len()
        }
    }

    let regex = TestRegex { locs: vec![0, 1, 2] };
    assert_eq!(regex.len(), 3);
}

#[test]
fn test_len_with_no_matches() {
    struct TestRegex {
        locs: Vec<usize>,
    }

    impl TestRegex {
        pub fn len(&self) -> usize {
            self.locs.len()
        }
    }

    let regex = TestRegex { locs: Vec::new() };
    assert_eq!(regex.len(), 0);
}

#[test]
fn test_len_with_empty_vector() {
    struct TestRegex {
        locs: Vec<usize>,
    }

    impl TestRegex {
        pub fn len(&self) -> usize {
            self.locs.len()
        }
    }

    let regex = TestRegex { locs: vec![] };
    assert_eq!(regex.len(), 0);
}

#[test]
fn test_len_with_one_empty_capture() {
    struct TestRegex {
        locs: Vec<usize>,
    }

    impl TestRegex {
        pub fn len(&self) -> usize {
            self.locs.len()
        }
    }

    let regex = TestRegex { locs: vec![0] }; // One capture group for the full match
    assert_eq!(regex.len(), 1);
}

