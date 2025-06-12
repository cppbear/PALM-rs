// Answer 0

#[test]
fn test_iter_non_empty_captures() {
    struct MockCaptures<'t> {
        text: &'t [u8],
        locs: Locations,
        named_groups: Arc<HashMap<String, usize>>,
    }

    let text = b"Test string for regex matching";
    let locs = Locations(vec![]);
    let named_groups = Arc::new(HashMap::new());
    let captures = MockCaptures { text, locs, named_groups };

    let sub_capture_matches = captures.iter();
    assert_eq!(sub_capture_matches.caps.text, captures.text);
}

#[test]
fn test_iter_empty_locations() {
    struct MockCaptures<'t> {
        text: &'t [u8],
        locs: Locations,
        named_groups: Arc<HashMap<String, usize>>,
    }

    let text = b"";
    let locs = Locations(vec![]);
    let named_groups = Arc::new(HashMap::new());
    let captures = MockCaptures { text, locs, named_groups };

    let sub_capture_matches = captures.iter();
    assert_eq!(sub_capture_matches.caps.text, captures.text);
}

#[test]
fn test_iter_single_location() {
    struct MockCaptures<'t> {
        text: &'t [u8],
        locs: Locations,
        named_groups: Arc<HashMap<String, usize>>,
    }

    let text = b"Single match";
    let locs = Locations(vec![Slot]);
    let named_groups = Arc::new(HashMap::new());
    let captures = MockCaptures { text, locs, named_groups };

    let sub_capture_matches = captures.iter();
    assert_eq!(sub_capture_matches.caps.text, captures.text);
}

#[should_panic]
fn test_iter_invalid_index() {
    struct MockCaptures<'t> {
        text: &'t [u8],
        locs: Locations,
        named_groups: Arc<HashMap<String, usize>>,
    }

    let text = b"Out of bounds test";
    let locs = Locations(vec![]);
    let named_groups = Arc::new(HashMap::new());
    let captures = MockCaptures { text, locs, named_groups };

    let sub_capture_matches = captures.iter();
    // Note: Here we would perform an operation that would cause a panic
    // This is illustrative only, as the panic would need to be defined by an actual use of sub_capture_matches
}

