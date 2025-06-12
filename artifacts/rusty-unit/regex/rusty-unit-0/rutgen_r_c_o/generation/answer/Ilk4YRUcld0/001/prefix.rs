// Answer 0

#[test]
fn test_expand_with_numeric_capture() {
    let text: &[u8] = b"hello world";
    let locs = Locations(vec![]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };
    let mut dst = Vec::new();
    
    captures.expand(b"$0", &mut dst);
}

#[test]
fn test_expand_with_named_capture() {
    let text: &[u8] = b"hello world";
    let locs = Locations(vec![]);
    let mut named_groups = HashMap::new();
    named_groups.insert("name".to_string(), 0);
    let captures = Captures { text, locs, named_groups: Arc::new(named_groups) };
    let mut dst = Vec::new();
    
    captures.expand(b"$name", &mut dst);
}

#[test]
fn test_expand_with_empty_replacement() {
    let text: &[u8] = b"hello world";
    let locs = Locations(vec![]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };
    let mut dst = Vec::new();
    
    captures.expand(b"", &mut dst);
}

#[test]
fn test_expand_with_literal_dollar_sign() {
    let text: &[u8] = b"hello world";
    let locs = Locations(vec![]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };
    let mut dst = Vec::new();
    
    captures.expand(b"$$", &mut dst);
}

#[test]
fn test_expand_with_invalid_name() {
    let text: &[u8] = b"hello world";
    let locs = Locations(vec![]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };
    let mut dst = Vec::new();
    
    captures.expand(b"$invalid_name", &mut dst);
}

#[test]
fn test_expand_with_multiple_replacements() {
    let text: &[u8] = b"hello world";
    let locs = Locations(vec![]);
    let mut named_groups = HashMap::new();
    named_groups.insert("name".to_string(), 0);
    let captures = Captures { text, locs, named_groups: Arc::new(named_groups) };
    let mut dst = Vec::new();
    
    captures.expand(b"$name and $0", &mut dst);
}

#[test]
fn test_expand_edge_case_large_replacement() {
    let text: &[u8] = b"hello world";
    let locs = Locations(vec![]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };
    let mut dst = Vec::new();
    
    let large_replacement = b"$0".repeat(256); // 256 * 4 = 1024 max
    captures.expand(&large_replacement, &mut dst);
}

#[test]
fn test_expand_with_just_braces() {
    let text: &[u8] = b"hello world";
    let locs = Locations(vec![]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };
    let mut dst = Vec::new();
    
    captures.expand(b"${}", &mut dst);
}

