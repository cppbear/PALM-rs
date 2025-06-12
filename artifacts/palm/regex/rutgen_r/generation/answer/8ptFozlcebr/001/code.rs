// Answer 0

#[test]
fn test_len_with_single_capture_group() {
    struct RegexMock {
        locs: Vec<usize>,
    }

    impl RegexMock {
        fn new(locations: Vec<usize>) -> Self {
            RegexMock { locs: locations }
        }
        
        fn len(&self) -> usize {
            self.locs.len()
        }
    }

    let regex = RegexMock::new(vec![0]); // single match
    assert_eq!(regex.len(), 1);
}

#[test]
fn test_len_with_multiple_capture_groups() {
    struct RegexMock {
        locs: Vec<usize>,
    }

    impl RegexMock {
        fn new(locations: Vec<usize>) -> Self {
            RegexMock { locs: locations }
        }
        
        fn len(&self) -> usize {
            self.locs.len()
        }
    }

    let regex = RegexMock::new(vec![0, 1, 2, 3]); // multiple matches
    assert_eq!(regex.len(), 4);
}

#[test]
fn test_len_with_no_capture_groups() {
    struct RegexMock {
        locs: Vec<usize>,
    }

    impl RegexMock {
        fn new(locations: Vec<usize>) -> Self {
            RegexMock { locs: locations }
        }
        
        fn len(&self) -> usize {
            self.locs.len()
        }
    }

    let regex = RegexMock::new(vec![]); // no matches
    assert_eq!(regex.len(), 0); // this should still return 0 since there are no matches.
}

#[test]
fn test_len_with_one_capture_group_and_multiple_empties() {
    struct RegexMock {
        locs: Vec<usize>,
    }

    impl RegexMock {
        fn new(locations: Vec<usize>) -> Self {
            RegexMock { locs: locations }
        }
        
        fn len(&self) -> usize {
            self.locs.len()
        }
    }

    let regex = RegexMock::new(vec![0, 0]); // multiple empty captures
    assert_eq!(regex.len(), 2); // still counts as two groups
}

#[test]
fn test_len_with_large_number_of_capture_groups() {
    struct RegexMock {
        locs: Vec<usize>,
    }

    impl RegexMock {
        fn new(locations: Vec<usize>) -> Self {
            RegexMock { locs: locations }
        }
        
        fn len(&self) -> usize {
            self.locs.len()
        }
    }

    let regex = RegexMock::new((0..1000).collect()); // 1000 captures
    assert_eq!(regex.len(), 1000);
}

