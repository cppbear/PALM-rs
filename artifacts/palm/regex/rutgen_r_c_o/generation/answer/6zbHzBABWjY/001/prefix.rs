// Answer 0

#[test]
fn test_get_with_valid_index_0() {
    let text = "abc123";
    let locs = Locations(vec![Some(0), Some(3)]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };
    let _ = captures.get(0);
}

#[test]
fn test_get_with_valid_index_1() {
    let text = "abc123";
    let locs = Locations(vec![Some(3), Some(6)]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };
    let _ = captures.get(1);
}

#[test]
fn test_get_with_valid_index_2() {
    let text = "abcXYZ";
    let locs = Locations(vec![Some(3), Some(6), Some(0), Some(3)]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };
    let _ = captures.get(2);
}

#[test]
fn test_get_with_invalid_index_10() {
    let text = "abc123";
    let locs = Locations(vec![Some(0), Some(3)]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };
    let _ = captures.get(10);
}

#[test]
fn test_get_with_invalid_index_100() {
    let text = "abc123";
    let locs = Locations(vec![Some(0), Some(3)]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };
    let _ = captures.get(100);
}

#[test]
fn test_get_with_max_usize() {
    let text = "abc123";
    let locs = Locations(vec![Some(0), Some(3)]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };
    let _ = captures.get(usize::MAX);
}

