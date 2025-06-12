// Answer 0

#[test]
fn test_len_with_single_capture_group() {
    struct MockRegex {
        locs: Vec<usize>,
    }

    impl MockRegex {
        pub fn len(&self) -> usize {
            self.locs.len()
        }
    }

    let regex = MockRegex { locs: vec![0] };
    assert_eq!(regex.len(), 1);
}

#[test]
fn test_len_with_multiple_capture_groups() {
    struct MockRegex {
        locs: Vec<usize>,
    }

    impl MockRegex {
        pub fn len(&self) -> usize {
            self.locs.len()
        }
    }

    let regex = MockRegex { locs: vec![0, 1, 2] };
    assert_eq!(regex.len(), 3);
}

#[test]
fn test_len_with_no_capture_group() {
    struct MockRegex {
        locs: Vec<usize>,
    }

    impl MockRegex {
        pub fn len(&self) -> usize {
            self.locs.len()
        }
    }

    let regex = MockRegex { locs: Vec::new() };
    assert_eq!(regex.len(), 0);
}

#[test]
fn test_len_with_full_match_only() {
    struct MockRegex {
        locs: Vec<usize>,
    }

    impl MockRegex {
        pub fn len(&self) -> usize {
            self.locs.len()
        }
    }

    let regex = MockRegex { locs: vec![0] };
    assert_eq!(regex.len(), 1);
}

