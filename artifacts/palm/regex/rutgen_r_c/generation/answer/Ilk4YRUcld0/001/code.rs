// Answer 0

#[test]
fn test_expand_empty_replacement() {
    let text = b"Hello, World!";
    let locs = Locations(Vec::new());
    let named_groups = Arc::new(HashMap::new());
    let caps = Captures { text, locs, named_groups };
    
    let mut dst = Vec::new();
    caps.expand(b"", &mut dst);
    
    assert_eq!(dst, b"");
}

#[test]
fn test_expand_no_captures() {
    let text = b"Hello, World!";
    let locs = Locations(Vec::new());
    let named_groups = Arc::new(HashMap::new());
    let caps = Captures { text, locs, named_groups };
    
    let mut dst = Vec::new();
    caps.expand(b"Test $1 and $name.", &mut dst);
    
    assert_eq!(dst, b"Test  and .");
}

#[test]
fn test_expand_with_valid_captures() {
    let text = b"Hello, World!";
    let loc1 = Slot { /* fill with appropriate data */ };
    let locs = Locations(vec![loc1]);
    let mut named_groups = HashMap::new();
    named_groups.insert("name".to_string(), 0);
    let named_groups = Arc::new(named_groups);
    let caps = Captures { text, locs, named_groups };
    
    let mut dst = Vec::new();
    caps.expand(b"Test $0 and ${name}.", &mut dst);
    
    assert_eq!(dst, b"Test Hello, World! and Hello, World!.");
}

#[test]
fn test_expand_with_invalid_capture() {
    let text = b"Hello, World!";
    let locs = Locations(Vec::new());
    let named_groups = Arc::new(HashMap::new());
    let caps = Captures { text, locs, named_groups };
    
    let mut dst = Vec::new();
    caps.expand(b"Test $2 and ${invalid}.", &mut dst);
    
    assert_eq!(dst, b"Test  and .");
}

#[test]
fn test_expand_with_double_dollar() {
    let text = b"Hello, World!";
    let locs = Locations(Vec::new());
    let named_groups = Arc::new(HashMap::new());
    let caps = Captures { text, locs, named_groups };
    
    let mut dst = Vec::new();
    caps.expand(b"Test $$ and $1.", &mut dst);
    
    assert_eq!(dst, b"Test $ and .");
}

#[test]
fn test_expand_with_nested_named_group() {
    let text = b"Hello, World!";
    let loc1 = Slot { /* fill with appropriate data */ };
    let locs = Locations(vec![loc1]);
    let mut named_groups = HashMap::new();
    named_groups.insert("name".to_string(), 0);
    let named_groups = Arc::new(named_groups);
    let caps = Captures { text, locs, named_groups };
    
    let mut dst = Vec::new();
    caps.expand(b"Test ${name} and ${name} again.", &mut dst);
    
    assert_eq!(dst, b"Test Hello, World! and Hello, World! again.");
}

