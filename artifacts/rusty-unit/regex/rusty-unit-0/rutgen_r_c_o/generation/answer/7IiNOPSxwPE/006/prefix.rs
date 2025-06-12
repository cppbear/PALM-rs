// Answer 0

#[test]
fn test_expand_str_basic() {
    let caps = re_unicode::Captures {
        text: "some text",
        locs: Locations::new(),
        named_groups: Arc::new(HashMap::new()),
    };
    let mut dst = String::new();
    expand_str(&caps, "some text$0more text", &mut dst);
}

#[test]
fn test_expand_str_with_named() {
    let mut named_groups = HashMap::new();
    named_groups.insert("greeting".to_string(), 1);
    let caps = re_unicode::Captures {
        text: "Hello, World!",
        locs: Locations::new(),
        named_groups: Arc::new(named_groups),
    };
    let mut dst = String::new();
    expand_str(&caps, "generic text$greeting$ other text", &mut dst);
}

#[test]
fn test_expand_str_empty_replacement() {
    let caps = re_unicode::Captures {
        text: "empty replacement",
        locs: Locations::new(),
        named_groups: Arc::new(HashMap::new()),
    };
    let mut dst = String::new();
    expand_str(&caps, "", &mut dst);
}

#[test]
fn test_expand_str_multiple_dollar() {
    let mut named_groups = HashMap::new();
    named_groups.insert("greeting".to_string(), 1);
    let caps = re_unicode::Captures {
        text: "Hello, World!",
        locs: Locations::new(),
        named_groups: Arc::new(named_groups),
    };
    let mut dst = String::new();
    expand_str(&caps, "$$gre$$eting$", &mut dst);
}

#[test]
fn test_expand_str_no_matches() {
    let caps = re_unicode::Captures {
        text: "Hello again!",
        locs: Locations::new(),
        named_groups: Arc::new(HashMap::new()),
    };
    let mut dst = String::new();
    expand_str(&caps, "text without match $unknown$", &mut dst);
}

