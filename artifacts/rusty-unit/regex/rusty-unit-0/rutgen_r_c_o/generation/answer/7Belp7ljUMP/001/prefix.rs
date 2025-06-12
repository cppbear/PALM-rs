// Answer 0

#[test]
fn test_expand_empty_replacement() {
    let caps = Captures {
        text: "test",
        locs: Locations(Vec::new()),
        named_groups: Arc::new(HashMap::new()),
    };
    let mut dst = String::new();
    caps.expand("", &mut dst);
}

#[test]
fn test_expand_single_named_capture() {
    let mut named_groups = HashMap::new();
    named_groups.insert("name".to_string(), 0);
    let caps = Captures {
        text: "test",
        locs: Locations(Vec::new()),
        named_groups: Arc::new(named_groups),
    };
    let mut dst = String::new();
    caps.expand("$name", &mut dst);
}

#[test]
fn test_expand_single_positional_capture() {
    let captures = vec!["test"];
    let mut named_groups = HashMap::new();
    let caps = Captures {
        text: "test",
        locs: Locations(captures.iter().enumerate().map(|(i, _)| Slot { index: i }).collect()),
        named_groups: Arc::new(named_groups),
    };
    let mut dst = String::new();
    caps.expand("$0", &mut dst);
}

#[test]
fn test_expand_multiple_captures() {
    let mut named_groups = HashMap::new();
    named_groups.insert("group1".to_string(), 0);
    named_groups.insert("group2".to_string(), 1);
    let caps = Captures {
        text: "result",
        locs: Locations(Vec::new()),
        named_groups: Arc::new(named_groups),
    };
    let mut dst = String::new();
    caps.expand("$group1 $group2", &mut dst);
}

#[test]
fn test_expand_invalid_capture() {
    let mut named_groups = HashMap::new();
    let caps = Captures {
        text: "test",
        locs: Locations(Vec::new()),
        named_groups: Arc::new(named_groups),
    };
    let mut dst = String::new();
    caps.expand("$invalid", &mut dst);
}

#[test]
fn test_expand_double_dollar() {
    let mut named_groups = HashMap::new();
    named_groups.insert("group".to_string(), 0);
    let caps = Captures {
        text: "value",
        locs: Locations(Vec::new()),
        named_groups: Arc::new(named_groups),
    };
    let mut dst = String::new();
    caps.expand("$$group", &mut dst);
}

#[test]
fn test_expand_brace_notation() {
    let mut named_groups = HashMap::new();
    named_groups.insert("1".to_string(), 0);
    let caps = Captures {
        text: "text",
        locs: Locations(Vec::new()),
        named_groups: Arc::new(named_groups),
    };
    let mut dst = String::new();
    caps.expand("${1}value", &mut dst);
}

#[test]
fn test_expand_long_string() {
    let mut named_groups = HashMap::new();
    named_groups.insert("long".to_string(), 0);
    let caps = Captures {
        text: "text",
        locs: Locations(Vec::new()),
        named_groups: Arc::new(named_groups),
    };
    let mut dst = String::new();
    let long_replacement = "This is a long test string with $long.";
    caps.expand(long_replacement, &mut dst);
}

