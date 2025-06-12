// Answer 0

#[test]
fn test_fmt_empty_captures() {
    let text: &str = "";
    let locs = Locations(vec![]);
    let named_groups: Arc<HashMap<String, usize>> = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };
    let debug_output = format!("{:?}", CapturesDebug(&captures));
}

#[test]
fn test_fmt_no_named_groups() {
    let text: &str = "abc";
    let locs = Locations(vec![]);
    let named_groups: Arc<HashMap<String, usize>> = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };
    let debug_output = format!("{:?}", CapturesDebug(&captures));
}

#[test]
fn test_fmt_single_slot_no_match() {
    let text: &str = "abc";
    let locs = Locations(vec![None]);
    let named_groups: Arc<HashMap<String, usize>> = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };
    let debug_output = format!("{:?}", CapturesDebug(&captures));
}

#[test]
fn test_fmt_single_slot_with_match() {
    let text: &str = "abc";
    let locs = Locations(vec![Some((0, 1))]);
    let named_groups: Arc<HashMap<String, usize>> = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };
    let debug_output = format!("{:?}", CapturesDebug(&captures));
}

#[test]
fn test_fmt_multiple_slots_no_names() {
    let text: &str = "abc";
    let locs = Locations(vec![Some((0, 1)), Some((1, 2)), None]);
    let named_groups: Arc<HashMap<String, usize>> = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };
    let debug_output = format!("{:?}", CapturesDebug(&captures));
}

#[test]
fn test_fmt_multiple_slots_with_names() {
    let text: &str = "abc";
    let locs = Locations(vec![Some((0, 1)), Some((1, 2)), None]);
    let mut named_groups_map = HashMap::new();
    named_groups_map.insert("first".to_string(), 0);
    named_groups_map.insert("second".to_string(), 1);
    let named_groups: Arc<HashMap<String, usize>> = Arc::new(named_groups_map);
    let captures = Captures { text, locs, named_groups };
    let debug_output = format!("{:?}", CapturesDebug(&captures));
}

