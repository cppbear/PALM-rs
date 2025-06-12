// Answer 0

#[test]
fn test_get_valid_capture() {
    struct TestLocations(Vec<Option<usize>>);

    impl Locations {
        fn new() -> Self {
            TestLocations(vec![Some(0), Some(3)]) // Simulating two captures (match at index 0 and 1)
        }
    }
    
    let text = b"abc123";
    let locs = TestLocations::new();
    let named_groups = Arc::new(HashMap::new());
    
    let captures = Captures {
        text,
        locs,
        named_groups,
    };

    assert_eq!(captures.get(0), Some(Match::new(text, 0, 3))); // Match for full "abc"
    assert_eq!(captures.get(1), None); // No valid match for index 1
}

#[test]
fn test_get_invalid_capture_index() {
    struct TestLocations(Vec<Option<usize>>);

    impl Locations {
        fn new() -> Self {
            TestLocations(vec![Some(0), Some(3)]) // Simulating two captures (match at index 0)
        }
    }
    
    let text = b"abc";
    let locs = TestLocations::new();
    let named_groups = Arc::new(HashMap::new());
    
    let captures = Captures {
        text,
        locs,
        named_groups,
    };

    assert_eq!(captures.get(2), None); // Out of bounds for captures
}

#[test]
fn test_get_empty_capture() {
    struct TestLocations(Vec<Option<usize>>);

    impl Locations {
        fn new() -> Self {
            TestLocations(vec![Some(2), None]) // First capture has match, second does not
        }
    }
    
    let text = b"abc";
    let locs = TestLocations::new();
    let named_groups = Arc::new(HashMap::new());
    
    let captures = Captures {
        text,
        locs,
        named_groups,
    };

    assert_eq!(captures.get(1), None); // Second capture does not match
}

