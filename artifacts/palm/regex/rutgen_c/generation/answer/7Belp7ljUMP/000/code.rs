// Answer 0

#[test]
fn test_expand_with_named_capture_group() {
    let text = "Hello, John!";
    let locs = Locations(vec![]);
    let named_groups = Arc::new(HashMap::from_iter(vec![("name".to_string(), 1)]));
    let captures = Captures { text, locs, named_groups };
    
    let mut dst = String::new();
    captures.expand("$name", &mut dst);
    
    assert_eq!(dst, "John");
}

#[test]
fn test_expand_with_index_capture_group() {
    let text = "Hello, John!";
    let locs = Locations(vec![]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };

    let mut dst = String::new();
    captures.expand("$0", &mut dst);
    
    assert_eq!(dst, "Hello, John!");
}

#[test]
fn test_expand_with_non_existing_named_group() {
    let text = "Hello, John!";
    let locs = Locations(vec![]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };

    let mut dst = String::new();
    captures.expand("$unknown", &mut dst);
    
    assert_eq!(dst, "");
}

#[test]
fn test_expand_with_literal_dollar() {
    let text = "Hello, John!";
    let locs = Locations(vec![]);
    let named_groups = Arc::new(HashMap::from_iter(vec![("name".to_string(), 1)]));
    let captures = Captures { text, locs, named_groups };

    let mut dst = String::new();
    captures.expand("Cost: $$100", &mut dst);
    
    assert_eq!(dst, "Cost: $100");
}

#[test]
fn test_expand_with_malformed_reference() {
    let text = "Hello, John!";
    let locs = Locations(vec![]);
    let named_groups = Arc::new(HashMap::from_iter(vec![("name".to_string(), 1)]));
    let captures = Captures { text, locs, named_groups };

    let mut dst = String::new();
    captures.expand("$1a", &mut dst);
    
    assert_eq!(dst, "a");
}

