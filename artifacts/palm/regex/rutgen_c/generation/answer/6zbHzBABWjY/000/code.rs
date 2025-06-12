// Answer 0

#[test]
fn test_get_valid_capture() {
    struct TestCaptures<'t> {
        text: &'t str,
        locs: Locations,
        named_groups: Arc<HashMap<String, usize>>,
    }

    impl<'t> Captures<'t> {
        fn new(text: &'t str) -> Self {
            let locs = Locations(vec![Some(0), Some(3), None]);
            let named_groups = Arc::new(HashMap::new());
            Captures { text, locs, named_groups }
        }
    }

    let caps = TestCaptures::new("abc123");
    if let Some(m) = caps.get(1) {
        assert_eq!(m.text, "abc123");
        assert_eq!(m.start, 3);
        assert_eq!(m.end, 6);
    } else {
        panic!("Expected a match for group 1");
    }
}

#[test]
fn test_get_invalid_capture() {
    struct TestCaptures<'t> {
        text: &'t str,
        locs: Locations,
        named_groups: Arc<HashMap<String, usize>>,
    }

    impl<'t> Captures<'t> {
        fn new(text: &'t str) -> Self {
            let locs = Locations(vec![Some(0), Some(3), None]);
            let named_groups = Arc::new(HashMap::new());
            Captures { text, locs, named_groups }
        }
    }

    let caps = TestCaptures::new("abc123");
    let result = caps.get(2);
    assert!(result.is_none(), "Expected no match for group 2");
}

#[test]
fn test_get_out_of_bounds_capture() {
    struct TestCaptures<'t> {
        text: &'t str,
        locs: Locations,
        named_groups: Arc<HashMap<String, usize>>,
    }

    impl<'t> Captures<'t> {
        fn new(text: &'t str) -> Self {
            let locs = Locations(vec![Some(0), Some(3), None]);
            let named_groups = Arc::new(HashMap::new());
            Captures { text, locs, named_groups }
        }
    }

    let caps = TestCaptures::new("abc123");
    let result = caps.get(3); // Out of bounds
    assert!(result.is_none(), "Expected no match for out-of-bounds group");
}

