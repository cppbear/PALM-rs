// Answer 0

#[test]
fn test_expand_basic_capture_group() {
    let text = "Hello, world!";
    let locs = Locations(vec![]);
    let named_groups: Arc<HashMap<String, usize>> = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };

    let mut dst = String::new();
    captures.expand("$0", &mut dst);
    assert_eq!(dst, "Hello, world!");
}

#[test]
fn test_expand_named_capture_group() {
    let text = "Hello, John!";
    let locs = Locations(vec![]);
    let mut named_groups = HashMap::new();
    named_groups.insert("name".to_string(), 1);
    let named_groups = Arc::new(named_groups);
    let captures = Captures { text, locs, named_groups };

    let mut dst = String::new();
    captures.expand("$name", &mut dst);
    assert_eq!(dst, "Hello, !"); // No actual match; expect empty replacement
}

#[test]
fn test_expand_with_nonexistent_group() {
    let text = "Hello, world!";
    let locs = Locations(vec![]);
    let named_groups: Arc<HashMap<String, usize>> = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };

    let mut dst = String::new();
    captures.expand("$nonexistent", &mut dst);
    assert_eq!(dst, ""); // Nonexistent group should lead to an empty output
}

#[test]
fn test_expand_with_double_dollar() {
    let text = "Hello, world!";
    let locs = Locations(vec![]);
    let named_groups: Arc<HashMap<String, usize>> = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };

    let mut dst = String::new();
    captures.expand("$$", &mut dst);
    assert_eq!(dst, "$"); // Double dollar should be converted to a single dollar
}

#[test]
fn test_expand_with_mixed_capture() {
    let text = "Hello, Alice!";
    let locs = Locations(vec![]);
    let mut named_groups = HashMap::new();
    named_groups.insert("user".to_string(), 1);
    let named_groups = Arc::new(named_groups);
    let captures = Captures { text, locs, named_groups };

    let mut dst = String::new();
    captures.expand("$user is here!", &mut dst);
    assert_eq!(dst, " is here!"); // User capture has no match
}

#[test]
fn test_expand_braced_capture_group() {
    let text = "Match one!";
    let locs = Locations(vec![]);
    let named_groups: Arc<HashMap<String, usize>> = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };

    let mut dst = String::new();
    captures.expand("${0} and more", &mut dst);
    assert_eq!(dst, "Match one! and more");
}

