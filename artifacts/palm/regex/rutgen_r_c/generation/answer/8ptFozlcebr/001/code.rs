// Answer 0

#[test]
fn test_captures_len_non_empty() {
    let locs = Locations(vec![Slot, Slot]); // Assuming Slot can be initialized this way
    let text: &[u8] = b"sample text";
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };

    assert_eq!(captures.len(), 1);
}

#[test]
fn test_captures_len_empty_locations() {
    let locs = Locations(vec![]);
    let text: &[u8] = b"";
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };

    assert_eq!(captures.len(), 0);
}

#[test]
fn test_captures_len_single_group() {
    let locs = Locations(vec![Slot]); // Representing one group match
    let text: &[u8] = b"match";
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };

    assert_eq!(captures.len(), 1);
}

#[test]
fn test_captures_len_multiple_groups() {
    let locs = Locations(vec![Slot, Slot, Slot]); // Assuming three groups
    let text: &[u8] = b"multiple matches";
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };

    assert_eq!(captures.len(), 3);
}

