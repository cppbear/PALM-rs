// Answer 0

#[test]
fn test_captures_len_single_group() {
    let text = "test";
    let locs = Locations(vec![Slot(0), Slot(4)]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };
    captures.len();
}

#[test]
fn test_captures_len_multiple_groups() {
    let text = "test input";
    let locs = Locations(vec![Slot(0), Slot(4), Slot(5), Slot(10)]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };
    captures.len();
}

#[test]
fn test_captures_len_with_named_groups() {
    let text = "my name is John";
    let locs = Locations(vec![Slot(0), Slot(14)]);
    let mut named_groups = HashMap::new();
    named_groups.insert("name".to_string(), 1);
    let captures = Captures { text, locs, named_groups: Arc::new(named_groups) };
    captures.len();
}

#[test]
fn test_captures_len_ten_groups() {
    let text = "multiple captures";
    let locs = Locations(vec![Slot(0), Slot(18), Slot(0), Slot(18), Slot(0), Slot(18), Slot(0), Slot(18), Slot(0), Slot(18)]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };
    captures.len();
}

#[test]
fn test_captures_len_edge_case() {
    let text = "";
    let locs = Locations(vec![Slot(0), Slot(0)]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };
    captures.len();
}

