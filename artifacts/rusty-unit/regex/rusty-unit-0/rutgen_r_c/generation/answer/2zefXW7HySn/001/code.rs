// Answer 0

#[test]
fn test_captures_len_basic() {
    let locs = Locations(vec![Slot, Slot]); // Assuming Slot is a valid type
    let captures = Captures {
        text: "hello",
        locs,
        named_groups: Arc::new(HashMap::new()),
    };
    assert_eq!(captures.len(), 1); // Basic test for at least one capture
}

#[test]
fn test_captures_len_multiple_groups() {
    let locs = Locations(vec![Slot, Slot, Slot, Slot]);
    let captures = Captures {
        text: "hello",
        locs,
        named_groups: Arc::new(HashMap::new()),
    };
    assert_eq!(captures.len(), 2); // Should count two groups
}

#[test]
fn test_captures_len_empty_groups() {
    let locs = Locations(vec![]);
    let captures = Captures {
        text: "",
        locs,
        named_groups: Arc::new(HashMap::new()),
    };
    assert_eq!(captures.len(), 0); // No groups present
}

#[test]
fn test_captures_len_single_group() {
    let locs = Locations(vec![Slot]);
    let captures = Captures {
        text: "test",
        locs,
        named_groups: Arc::new(HashMap::new()),
    };
    assert_eq!(captures.len(), 1); // One capture group present
}

