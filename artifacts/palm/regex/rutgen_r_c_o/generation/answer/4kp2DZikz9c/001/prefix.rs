// Answer 0

#[test]
fn test_iter_empty_captures() {
    let named_groups = Arc::new(HashMap::new());
    let locs = Locations(Vec::new());
    let text: &[u8] = b"";
    let captures = Captures { text, locs, named_groups };
    let _ = captures.iter();
}

#[test]
fn test_iter_single_capture() {
    let mut named_groups = HashMap::new();
    named_groups.insert("group1".to_string(), 0);
    let named_groups = Arc::new(named_groups);
    let locs = Locations(vec![Slot { start: 0, end: 3 }]); // assume Slot is initialized appropriately
    let text: &[u8] = b"abc";
    let captures = Captures { text, locs, named_groups };
    let _ = captures.iter();
}

#[test]
fn test_iter_multiple_captures() {
    let mut named_groups = HashMap::new();
    named_groups.insert("group1".to_string(), 0);
    named_groups.insert("group2".to_string(), 1);
    let named_groups = Arc::new(named_groups);
    let locs = Locations(vec![
        Slot { start: 0, end: 3 }, // match for group1
        Slot { start: 4, end: 7 }  // match for group2
    ]); // assume Slot is initialized appropriately
    let text: &[u8] = b"abc def";
    let captures = Captures { text, locs, named_groups };
    let _ = captures.iter();
}

#[test]
fn test_iter_no_named_groups() {
    let named_groups = Arc::new(HashMap::new());
    let locs = Locations(vec![Slot { start: 0, end: 5 }]); // assume Slot is initialized appropriately
    let text: &[u8] = b"hello";
    let captures = Captures { text, locs, named_groups };
    let _ = captures.iter();
}

#[test]
fn test_iter_large_text() {
    let mut named_groups = HashMap::new();
    named_groups.insert("group1".to_string(), 0);
    let named_groups = Arc::new(named_groups);
    let locs = Locations(vec![
        Slot { start: 0, end: 10 },
        Slot { start: 11, end: 20 }
    ]); // assume Slot is initialized appropriately
    let text: &[u8] = b"abcdefghij klmnopqrst";
    let captures = Captures { text, locs, named_groups };
    let _ = captures.iter();
}

