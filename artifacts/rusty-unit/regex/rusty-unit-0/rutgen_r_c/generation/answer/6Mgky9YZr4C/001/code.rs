// Answer 0

#[test]
fn test_get_with_valid_capture_group() {
    let text = b"abc123";
    let locs = Locations(vec![Some(0), Some(3), Some(3), Some(6)]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };

    let match1 = captures.get(1);
    assert!(match1.is_some());
    let m1 = match1.unwrap();
    assert_eq!(m1.start, 3);
    assert_eq!(m1.end, 6);
    assert_eq!(m1.text, b"abc123");

    let match2 = captures.get(2);
    assert!(match2.is_none());
}

#[test]
fn test_get_with_invalid_capture_group() {
    let text = b"abc123";
    let locs = Locations(vec![Some(0), Some(3), Some(3), Some(6)]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };

    let match_out_of_bounds = captures.get(3);
    assert!(match_out_of_bounds.is_none());

    let match_negative_index = captures.get(usize::MAX);
    assert!(match_negative_index.is_none());
}

#[test]
fn test_get_with_no_matches() {
    let text = b"xyz";
    let locs = Locations(vec![None, None, None, None]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };

    let match1 = captures.get(1);
    assert!(match1.is_none());

    let match2 = captures.get(2);
    assert!(match2.is_none());
}

