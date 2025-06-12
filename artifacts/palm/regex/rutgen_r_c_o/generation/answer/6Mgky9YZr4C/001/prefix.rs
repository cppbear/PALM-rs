// Answer 0

#[test]
fn test_get_valid_capture_1() {
    let text = b"abc123";
    let locs = Locations(vec![Some(0), Some(3)]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };
    let _ = captures.get(0);
}

#[test]
fn test_get_valid_capture_2() {
    let text = b"abc123";
    let locs = Locations(vec![Some(0), Some(3), Some(6), Some(8)]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };
    let _ = captures.get(1);
}

#[test]
fn test_get_invalid_capture() {
    let text = b"abc123";
    let locs = Locations(vec![Some(0), Some(3)]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };
    let _ = captures.get(2);
}

#[test]
fn test_get_empty_locations() {
    let text = b"";
    let locs = Locations(vec![]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };
    let _ = captures.get(0);
}

#[test]
fn test_get_single_capture() {
    let text = b"abc";
    let locs = Locations(vec![Some(0), Some(3)]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };
    let _ = captures.get(0);
}

