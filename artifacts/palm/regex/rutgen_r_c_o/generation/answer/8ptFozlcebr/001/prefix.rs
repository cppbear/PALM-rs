// Answer 0

#[test]
fn test_len_single_capture() {
    let locs = Locations(vec![Slot { start: 0, end: 1 }]);
    let captures = Captures {
        text: b"test",
        locs,
        named_groups: Arc::new(HashMap::new()),
    };
    let _ = captures.len();
}

#[test]
fn test_len_multiple_captures() {
    let locs = Locations(vec![Slot { start: 0, end: 1 }, Slot { start: 1, end: 2 }]);
    let captures = Captures {
        text: b"test",
        locs,
        named_groups: Arc::new(HashMap::new()),
    };
    let _ = captures.len();
}

#[test]
fn test_len_no_captures() {
    let locs = Locations(vec![Slot { start: 0, end: 1 }]); // At least one capture for the full match.
    let captures = Captures {
        text: b"test",
        locs,
        named_groups: Arc::new(HashMap::new()),
    };
    let _ = captures.len();
}

#[test]
fn test_len_edge_cases() {
    let locs = Locations(vec![Slot { start: 0, end: 1 }; 1000]); // Maximum captures allowable.
    let captures = Captures {
        text: b"test",
        locs,
        named_groups: Arc::new(HashMap::new()),
    };
    let _ = captures.len();
}

#[test]
fn test_len_empty_text() {
    let locs = Locations(vec![Slot { start: 0, end: 0 }]); // Empty text still corresponds to a full match.
    let captures = Captures {
        text: b"",
        locs,
        named_groups: Arc::new(HashMap::new()),
    };
    let _ = captures.len();
}

